use std::collections::HashMap;

use source::map::TextRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprId(pub usize);

#[derive(Debug, Clone)]
pub enum Literal {
    Bootstrap(char),
    Source(TextRef),
}

pub enum ExprBuilder {
    Literal(Literal),
    Seq(Vec<ExprBuilder>),
    Alt(Vec<ExprBuilder>),
    Opt(Box<ExprBuilder>),
    Star(Box<ExprBuilder>),
    Plus(Box<ExprBuilder>),
    Ref(RuleId),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Seq(Vec<ExprId>),
    Alt(Vec<ExprId>),
    Opt(ExprId),
    Star(ExprId),
    Plus(ExprId),
    Ref(RuleId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuleId(pub usize);

#[derive(Debug, Clone)]
pub struct Rule {
    expression: ExprId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RuleName {
    Bootstrap(&'static str),
    Source(TextRef),
}

#[derive(Debug, Default, Clone)]
pub struct RuleRegistry {
    rules: HashMap<RuleId, Rule>,
    names: HashMap<RuleName, RuleId>,
    exprs: Vec<Expr>,
    next_id: usize,
}

impl RuleRegistry {
    pub fn declare_builtin(&mut self, name: &'static str) -> RuleId {
        self.declare_rule(RuleName::Bootstrap(name))
    }

    pub fn declare_rule(&mut self, name: RuleName) -> RuleId {
        let id = RuleId(self.next_id);
        self.next_id += 1;

        self.names.insert(name, id);
        id
    }

    pub fn define_builtin(
        &mut self,
        rule: RuleId,
        build: impl FnOnce(&RuleRegistry) -> Option<ExprBuilder>,
    ) -> Option<RuleId> {
        let expression = build(self)?;
        let id = self.compile(expression);
        self.define_rule(rule, id)
    }

    fn define_rule(&mut self, id: RuleId, expression: ExprId) -> Option<RuleId> {
        self.rules.insert(id, Rule { expression });
        Some(id)
    }

    fn add_expr(&mut self, expr: Expr) -> ExprId {
        let id = ExprId(self.exprs.len());
        self.exprs.push(expr);
        id
    }
}

impl RuleRegistry {
    pub fn ref_rule(&self, rule: RuleId) -> ExprBuilder {
        ExprBuilder::Ref(rule)
    }

    pub fn ref_builtin(&self, name: &'static str) -> Option<ExprBuilder> {
        Some(ExprBuilder::Ref(
            *self.names.get(&RuleName::Bootstrap(name))?,
        ))
    }

    pub fn string(&self, s: &'static str) -> ExprBuilder {
        ExprBuilder::Seq(
            s.chars()
                .map(|c| ExprBuilder::Literal(Literal::Bootstrap(c)))
                .collect(),
        )
    }

    pub fn literal(&self, c: char) -> ExprBuilder {
        ExprBuilder::Literal(Literal::Bootstrap(c))
    }

    pub fn alt(&self, expressions: impl IntoIterator<Item = ExprBuilder>) -> ExprBuilder {
        ExprBuilder::Alt(expressions.into_iter().collect())
    }

    pub fn seq(&self, expressions: impl IntoIterator<Item = ExprBuilder>) -> ExprBuilder {
        ExprBuilder::Seq(expressions.into_iter().collect())
    }

    pub fn opt(&self, expression: ExprBuilder) -> ExprBuilder {
        ExprBuilder::Opt(Box::new(expression))
    }

    pub fn star(&self, expression: ExprBuilder) -> ExprBuilder {
        ExprBuilder::Star(Box::new(expression))
    }

    pub fn plus(&self, expression: ExprBuilder) -> ExprBuilder {
        ExprBuilder::Plus(Box::new(expression))
    }

    pub fn compile(&mut self, builder: ExprBuilder) -> ExprId {
        let expr = match builder {
            ExprBuilder::Literal(literal) => Expr::Literal(literal),
            ExprBuilder::Seq(items) => {
                Expr::Seq(items.into_iter().map(|item| self.compile(item)).collect())
            }
            ExprBuilder::Alt(items) => {
                Expr::Alt(items.into_iter().map(|item| self.compile(item)).collect())
            }
            ExprBuilder::Opt(item) => Expr::Opt(self.compile(*item)),
            ExprBuilder::Star(item) => Expr::Star(self.compile(*item)),
            ExprBuilder::Plus(item) => Expr::Plus(self.compile(*item)),
            ExprBuilder::Ref(rule) => Expr::Ref(rule),
        };
        self.add_expr(expr)
    }
}

impl RuleRegistry {
    pub fn print_bootstrap(&self) {
        print!("{}", self.write_bootstrap())
    }

    fn write_bootstrap(&self) -> String {
        let mut rules: Vec<_> = self
            .names
            .iter()
            .filter_map(|(name, id)| match name {
                RuleName::Bootstrap(name) if self.rules.contains_key(id) => Some((*id, *name)),
                _ => None,
            })
            .collect();

        rules.sort_by_key(|(RuleId(index), _)| *index);

        let mut output = String::new();

        for (id, name) in rules {
            let expression = self.rules[&id].expression;
            output.push_str(&format!(
                "{} := {}\n",
                name.to_ascii_uppercase(),
                self.format_expr(expression, 0)
            ));
        }

        output
    }

    fn format_expr(&self, id: ExprId, parent_precedence: u8) -> String {
        let ExprId(index) = id;
        let (precedence, result) = match &self.exprs[index] {
            Expr::Literal(Literal::Bootstrap(c)) => (4, format!("{:?}", c.to_string())),
            Expr::Literal(Literal::Source(source)) => (4, format!("<source:{:?}>", source)),
            Expr::Ref(RuleId(rule)) => {
                let name = self
                    .names
                    .iter()
                    .find_map(|(name, RuleId(id))| {
                        if id == rule {
                            Some(match name {
                                RuleName::Bootstrap(name) => name.to_ascii_uppercase(),
                                RuleName::Source(source) => format!("{:?}", source),
                            })
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| format!("<rule:{}>", rule));
                (4, name)
            }
            Expr::Seq(items) => {
                let mut parts = Vec::new();
                let mut literal = String::new();

                for &ExprId(item) in items {
                    match &self.exprs[item] {
                        Expr::Literal(Literal::Bootstrap(c)) => {
                            literal.push(*c);
                        }
                        _ => {
                            if !literal.is_empty() {
                                parts.push(format!("{:?}", literal));
                                literal.clear();
                            }

                            parts.push(self.format_expr(ExprId(item), 2));
                        }
                    }
                }
                if !literal.is_empty() {
                    parts.push(format!("{:?}", literal));
                }
                (2, parts.join(" "))
            }
            Expr::Alt(items) => {
                let parts: Vec<_> = items
                    .iter()
                    .map(|item| self.format_expr(*item, 1))
                    .collect();

                (1, parts.join(" | "))
            }

            Expr::Opt(item) => (3, format!("{}?", self.format_expr(*item, 3))),

            Expr::Star(item) => (3, format!("{}*", self.format_expr(*item, 3))),

            Expr::Plus(item) => (3, format!("{}+", self.format_expr(*item, 3))),
        };

        if precedence < parent_precedence {
            format!("({result})")
        } else {
            result
        }
    }
}
