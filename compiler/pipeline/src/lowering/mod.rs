use surface::{
    BinderGroup, Constructor, Declaration, Definition, ExprKind, IdentId, Inductive, Level, Module,
    Plicity, QualifiedIdent, Sort, SurfaceExpr,
};
use syntax::{
    grammar::RuleName,
    node::{Node, NodeKind},
    source::Spanned,
    token::TokenKind,
};

use crate::{diagnostics::Diagnostic, lowering::interner::IdentInterner};

mod interner;

fn partition_children(
    children: Vec<Node>,
    predicate: impl Fn(&NodeKind) -> bool,
) -> (Vec<Node>, Vec<Node>) {
    let mut matched = Vec::new();
    let mut remaining = Vec::new();

    for child in children {
        if predicate(&child.kind) {
            matched.push(child);
        } else {
            remaining.push(child);
        }
    }

    (matched, remaining)
}

fn take_one(
    children: Vec<Node>,
    predicate: impl Fn(&NodeKind) -> bool,
) -> Result<Node, Diagnostic> {
    let (matched, _) = partition_children(children, predicate);
    matched.into_iter().next().ok_or_else(|| todo!())
}

#[derive(Default)]
pub struct Lowerer {
    interner: IdentInterner,
}

impl Lowerer {
    pub fn lower(&mut self, root: Node) -> Result<Module, Vec<Diagnostic>> {
        let mut declarations = Vec::new();
        let mut diagnostics = Vec::new();

        for child in root.children {
            match self.lower_declaration(child) {
                Ok(declaration) => declarations.push(declaration),
                Err(error) => diagnostics.push(error),
            }
        }

        if diagnostics.is_empty() {
            Ok(Module { declarations })
        } else {
            Err(diagnostics)
        }
    }

    pub fn lower_universe_expression(&mut self, node: Node) -> Result<Level, Diagnostic> {
        let Node { kind, children, .. } = node;

        match kind {
            NodeKind::Rule(RuleName::UniverseLevel) => {
                let mut children = children.into_iter();

                let first = children.next().ok_or_else(|| todo!())?;

                match &first.kind {
                    NodeKind::Token(TokenKind::Ident) => Ok(Level::Ident(self.lower_ident(first)?)),
                    NodeKind::Token(TokenKind::Zero) => Ok(Level::Zero),
                    NodeKind::Token(TokenKind::Succ) => {
                        let inner = take_one(children.collect(), |kind| {
                            matches!(kind, NodeKind::Rule(RuleName::UniverseLevel))
                        })?;

                        Ok(Level::Succ(Box::new(
                            self.lower_universe_expression(inner)?,
                        )))
                    }
                    NodeKind::Token(TokenKind::Max) => {
                        let (levels, _) = partition_children(children.collect(), |kind| {
                            matches!(kind, NodeKind::Rule(RuleName::UniverseLevel))
                        });

                        if levels.len() != 2 {
                            todo!();
                        }

                        let mut levels = levels.into_iter();

                        let lhs = levels.next().ok_or_else(|| todo!())?;
                        let rhs = levels.next().ok_or_else(|| todo!())?;

                        Ok(Level::Max(
                            Box::new(self.lower_universe_expression(lhs)?),
                            Box::new(self.lower_universe_expression(rhs)?),
                        ))
                    }
                    NodeKind::Rule(RuleName::ParenLevel) => self.lower_universe_expression(first),
                    _ => todo!(),
                }
            }

            NodeKind::Rule(RuleName::ParenLevel) => {
                let inner = take_one(children, |kind| {
                    matches!(kind, NodeKind::Rule(RuleName::UniverseLevel))
                })?;

                self.lower_universe_expression(inner)
            }

            _ => todo!(),
        }
    }

    fn lower_sort_expression(&mut self, node: Node) -> Result<SurfaceExpr, Diagnostic> {
        let level = node
            .children
            .into_iter()
            .find(|child| matches!(&child.kind, NodeKind::Rule(RuleName::UniverseLevel)))
            .map(|level| self.lower_universe_expression(level))
            .transpose()?;

        Ok(Spanned::new(ExprKind::Sort(Sort::Sort(level)), node.span))
    }

    fn lower_ident(&mut self, node: Node) -> Result<IdentId, Diagnostic> {
        match node.kind {
            NodeKind::Token(TokenKind::Ident) => Ok(self.interner.intern(node.span)),
            _ => todo!(),
        }
    }
}

impl Lowerer {
    fn lower_expression(&mut self, node: Node) -> Result<SurfaceExpr, Diagnostic> {
        let s = match node.kind {
            NodeKind::Token(TokenKind::Prop) => Spanned::new(ExprKind::Sort(Sort::Prop), node.span),
            NodeKind::Token(TokenKind::SelfType) => Spanned::new(ExprKind::SelfType, node.span),
            NodeKind::Rule(RuleName::SortExpr) => self.lower_sort_expression(node)?,
            NodeKind::Rule(RuleName::Expr) => self.lower_arrow_expression(node)?,
            NodeKind::Rule(RuleName::AppExpr) => self.lower_application(node)?,
            NodeKind::Rule(RuleName::QualifiedIdent) => self.lower_qualified_ident(node)?,
            NodeKind::Rule(RuleName::ParenExpr) => self.lower_parenthesized(node)?,
            NodeKind::Rule(RuleName::Lambda) => self.lower_lambda(node)?,
            NodeKind::Rule(RuleName::Forall) => self.lower_forall(node)?,
            _ => todo!(),
        };
        Ok(s)
    }
}

impl Lowerer {
    fn lower_parenthesized(&mut self, node: Node) -> Result<SurfaceExpr, Diagnostic> {
        let Node { children, .. } = node;

        let mut expression = None;

        for child in children {
            match child.kind {
                NodeKind::Rule(RuleName::Expr) => expression = Some(child),
                _ => {}
            }
        }

        let expression = expression.ok_or_else(|| todo!())?;
        self.lower_expression(expression)
    }
}

impl Lowerer {
    fn lower_application(&mut self, node: Node) -> Result<SurfaceExpr, Diagnostic> {
        let mut expressions = node
            .children
            .into_iter()
            .filter(|child| matches!(&child.kind, NodeKind::Rule(RuleName::AtomicExpr)))
            .map(|child| self.lower_atomic_expression(child));

        let mut result = expressions.next().ok_or_else(|| todo!())??;

        for expression in expressions {
            result = Spanned::new(
                ExprKind::Application {
                    function: Box::new(result),
                    argument: Box::new(expression?),
                },
                node.span,
            );
        }

        Ok(result)
    }
}

impl Lowerer {
    fn lower_atomic_expression(&mut self, node: Node) -> Result<SurfaceExpr, Diagnostic> {
        let Node { children, .. } = node;

        let mut expr = None;
        for child in children {
            expr = Some(child);
            break;
        }

        let expr = expr.ok_or_else(|| todo!())?;

        self.lower_expression(expr)
    }
}

impl Lowerer {
    fn lower_arrow_expression(&mut self, node: Node) -> Result<SurfaceExpr, Diagnostic> {
        let mut operands = node
            .children
            .into_iter()
            .filter(|child| matches!(&child.kind, NodeKind::Rule(RuleName::AppExpr)))
            .map(|child| self.lower_expression(child))
            .collect::<Result<Vec<_>, _>>()?;

        let mut result = operands.pop().ok_or_else(|| todo!())?;

        while let Some(left) = operands.pop() {
            let binder = Spanned::new(
                BinderGroup {
                    names: vec![],
                    ty: Some(left),
                    plicity: Plicity::Explicit,
                },
                node.span,
            );

            result = Spanned::new(
                ExprKind::Pi {
                    binders: vec![binder],
                    body: Box::new(result),
                },
                node.span,
            );
        }

        Ok(result)
    }
}

impl Lowerer {
    fn lower_binder_group(&mut self, node: Node) -> Result<Spanned<BinderGroup>, Diagnostic> {
        let Node {
            kind,
            children,
            span,
        } = node;

        let plicity = match kind {
            NodeKind::Rule(RuleName::ExplicitSection) => Plicity::Explicit,
            NodeKind::Rule(RuleName::ImplicitSection) => Plicity::Implicit,
            _ => todo!(),
        };

        let mut names = Vec::new();

        let mut type_node = None;

        for child in children {
            match child.kind {
                NodeKind::Token(TokenKind::Ident) => names.push(self.lower_ident(child)?),
                NodeKind::Rule(RuleName::TypeSpec) => type_node = Some(child),
                _ => {}
            }
        }

        if names.is_empty() {
            todo!()
        }

        let type_node = type_node.ok_or_else(|| todo!())?;

        Ok(Spanned::new(
            BinderGroup {
                names,
                ty: Some(self.lower_type_spec(type_node)?),
                plicity,
            },
            span,
        ))
    }
}

impl Lowerer {
    fn lower_parameters(&mut self, node: Node) -> Result<Vec<Spanned<BinderGroup>>, Diagnostic> {
        node.children
            .into_iter()
            .filter(|child| {
                matches!(
                    &child.kind,
                    NodeKind::Rule(RuleName::ExplicitSection | RuleName::ImplicitSection)
                )
            })
            .map(|child| self.lower_binder_group(child))
            .collect()
    }
}
impl Lowerer {
    fn lower_lambda(&mut self, node: Node) -> Result<SurfaceExpr, Diagnostic> {
        let Node { children, span, .. } = node;

        let mut params = None;
        let mut body = None;

        for child in children {
            match child.kind {
                NodeKind::Rule(RuleName::Params) => params = Some(child),
                NodeKind::Rule(RuleName::Expr) => body = Some(child),
                _ => {}
            }
        }

        let params = params.ok_or_else(|| todo!())?;
        let body = body.ok_or_else(|| todo!())?;

        Ok(Spanned::new(
            ExprKind::Lambda {
                binders: self.lower_parameters(params)?,
                body: Box::new(self.lower_expression(body)?),
            },
            span,
        ))
    }
}

impl Lowerer {
    fn lower_forall(&mut self, node: Node) -> Result<SurfaceExpr, Diagnostic> {
        let Node { children, span, .. } = node;

        let mut params = None;
        let mut body = None;

        for child in children {
            match child.kind {
                NodeKind::Rule(RuleName::Params) => params = Some(child),
                NodeKind::Rule(RuleName::Expr) => body = Some(child),
                _ => {}
            }
        }

        let params = params.ok_or_else(|| todo!())?;
        let body = body.ok_or_else(|| todo!())?;

        Ok(Spanned::new(
            ExprKind::Pi {
                binders: self.lower_parameters(params)?,
                body: Box::new(self.lower_expression(body)?),
            },
            span,
        ))
    }

    fn lower_type_spec(&mut self, node: Node) -> Result<SurfaceExpr, Diagnostic> {
        let Node { children, .. } = node;

        let mut expr = None;

        for child in children {
            match child.kind {
                NodeKind::Rule(RuleName::Expr) => expr = Some(child),
                _ => {}
            }
        }

        let expr = expr.ok_or_else(|| todo!())?;

        self.lower_expression(expr)
    }
}

impl Lowerer {
    fn lower_declaration(&mut self, node: Node) -> Result<Spanned<Declaration>, Diagnostic> {
        let span = node.span;

        let decl = match &node.kind {
            NodeKind::Rule(RuleName::Const) => {
                Declaration::Definition(self.lower_definition(node)?)
            }
            NodeKind::Rule(RuleName::Ind) => Declaration::Inductive(self.lower_inductive(node)?),
            _ => todo!(),
        };

        Ok(Spanned::new(decl, span))
    }
}

impl Lowerer {
    fn lower_definition(&mut self, node: Node) -> Result<Definition, Diagnostic> {
        let Node { children, .. } = node;

        let mut name = None;
        let mut params = None;
        let mut type_spec = None;
        let mut value = None;

        for child in children {
            match child.kind {
                NodeKind::Token(TokenKind::Ident) => name = Some(child),
                NodeKind::Rule(RuleName::Params) => params = Some(child),
                NodeKind::Rule(RuleName::TypeSpec) => type_spec = Some(child),
                NodeKind::Rule(RuleName::Expr) => value = Some(child),
                _ => {}
            }
        }

        let name = name.ok_or_else(|| todo!())?;
        let params = params.ok_or_else(|| todo!())?;
        let type_spec = type_spec.ok_or_else(|| todo!())?;
        let value = value.ok_or_else(|| todo!())?;

        Ok(Definition {
            name: self.lower_ident(name)?,
            parameters: self.lower_parameters(params)?,
            ty: Some(self.lower_type_spec(type_spec)?),
            value: self.lower_expression(value)?,
        })
    }
}

impl Lowerer {
    fn lower_inductive(&mut self, node: Node) -> Result<Inductive, Diagnostic> {
        let Node { children, .. } = node;

        let mut name = None;
        let mut params = None;
        let mut type_spec = None;
        let mut constructors = Vec::new();
        let mut recursor_name = None;

        for child in children {
            match child.kind {
                NodeKind::Token(TokenKind::Ident) if name.is_none() => name = Some(child),
                NodeKind::Rule(RuleName::Params) => params = Some(child),
                NodeKind::Rule(RuleName::TypeSpec) => type_spec = Some(child),
                NodeKind::Rule(RuleName::Constructor) => {
                    constructors.push(self.lower_constructor(child)?)
                }
                NodeKind::Token(TokenKind::Ident) => recursor_name = Some(child),
                _ => {}
            }
        }

        let name = name.ok_or_else(|| todo!())?;
        let params = params.ok_or_else(|| todo!())?;
        let type_spec = type_spec.ok_or_else(|| todo!())?;
        let recursor_name = recursor_name.ok_or_else(|| todo!())?;

        Ok(Inductive {
            name: self.lower_ident(name)?,
            parameters: self.lower_parameters(params)?,
            ty: Some(self.lower_type_spec(type_spec)?),
            cons: constructors,
            recursor_name: Some(self.lower_ident(recursor_name)?),
        })
    }
}

impl Lowerer {
    fn lower_constructor(&mut self, node: Node) -> Result<Spanned<Constructor>, Diagnostic> {
        let Node { children, .. } = node;

        let mut name = None;
        let mut params = None;
        let mut type_spec = None;

        for child in children {
            match child.kind {
                NodeKind::Token(TokenKind::Ident) => name = Some(child),
                NodeKind::Rule(RuleName::Params) => params = Some(child),
                NodeKind::Rule(RuleName::TypeSpec) => type_spec = Some(child),
                _ => {}
            }
        }

        let name = name.ok_or_else(|| todo!())?;
        let params = params.ok_or_else(|| todo!())?;
        let type_spec = type_spec.ok_or_else(|| todo!())?;

        Ok(Spanned::new(
            Constructor {
                name: self.lower_ident(name)?,
                parameters: self.lower_parameters(params)?,
                ty: self.lower_type_spec(type_spec)?,
            },
            node.span,
        ))
    }
}

impl Lowerer {
    fn lower_qualified_ident(&mut self, node: Node) -> Result<SurfaceExpr, Diagnostic> {
        let Node { children, span, .. } = node;

        let segments = children
            .into_iter()
            .filter(|child| matches!(&child.kind, NodeKind::Token(TokenKind::Ident)))
            .map(|child| self.lower_ident(child))
            .collect::<Result<Vec<_>, _>>()?;

        if segments.is_empty() {
            todo!()
        }

        Ok(Spanned::new(
            ExprKind::Identifier(QualifiedIdent { segments }),
            span,
        ))
    }
}
