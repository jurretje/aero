use serde::{Deserialize, Serialize};
use syntax::{
    grammar::build_grammar,
    graph::{GrammarGraph, GrammarGraphBuilder},
    lexer::Lexer,
    node::Node,
    parser::{ParseCosts, build_ast, parse},
    source::Span,
    theme::{Style, ThemeName},
    token::TokenKind,
};

pub struct Compiler {
    pub options: CompilerOptions,
    pub grammar_graph: GrammarGraph,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerOptions {
    pub parse_costs: ParseCosts,
    pub theme: ThemeName,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self {
            parse_costs: ParseCosts {
                insert: 2,
                delete: 4,
                replace: 5,
            },
            theme: ThemeName::TokyoNight,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct StylizedToken {
    kind: TokenKind,
    span: Span,
    style: Style,
}

#[derive(Debug, Serialize)]
pub struct Analysis {
    pub tokens: Vec<StylizedToken>,
    pub parsed: Option<Node>,
}

pub struct CompileOutput {
    pub parsed: Option<Node>,
}

impl Compiler {
    pub fn new() -> Self {
        let grammar_graph = GrammarGraphBuilder::new(&build_grammar()).build();

        Self {
            options: CompilerOptions::default(),
            grammar_graph,
        }
    }

    pub fn analyze(&self, source: &str) -> Analysis {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokens();

        let parsed = parse(&self.grammar_graph, &tokens, self.options.parse_costs)
            .and_then(|actions| build_ast(&actions, &tokens));

        Analysis {
            tokens: tokens
                .iter()
                .map(|tok| {
                    let style = tok.kind.style();
                    StylizedToken {
                        kind: tok.kind,
                        span: tok.span.clone(),
                        style,
                    }
                })
                .collect(),
            parsed,
        }
    }

    pub fn compile(&self, source: &str) -> CompileOutput {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokens();

        let parsed = parse(&self.grammar_graph, &tokens, self.options.parse_costs)
            .and_then(|actions| build_ast(&actions, &tokens));

        CompileOutput { parsed }
    }
}
