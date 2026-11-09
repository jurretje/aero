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

use crate::diagnostics::Diagnostic;

pub mod diagnostics;
pub mod lowering;

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
            theme: ThemeName::Feather,
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
    pub surface: Option<surface::Module>,
    pub diagnostics: Vec<Diagnostic>,
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
        let (tokens, _) = lexer.tokens();

        let parser_output = parse(&self.grammar_graph, &tokens, self.options.parse_costs);

        let ast = parser_output
            .actions
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
            parsed: ast,
        }
    }

    pub fn compile(&self, source: &str) -> CompileOutput {
        let mut lexer = Lexer::new(source);
        let (tokens, lex_errors) = lexer.tokens();

        dbg!(lex_errors);
        dbg!(&tokens);

        let parser_output = parse(&self.grammar_graph, &tokens, self.options.parse_costs);

        let Some(actions) = parser_output.actions else {
            return CompileOutput {
                parsed: None,
                surface: None,
                diagnostics: vec![],
            };
        };

        let syntax = build_ast(&actions, &tokens);

        let Some(syntax) = syntax else {
            return CompileOutput {
                parsed: None,
                surface: None,
                diagnostics: vec![],
            };
        };

        let mut diagnostics = Vec::new();

        let surface = match lowering::lower(&syntax, source) {
            Ok(module) => Some(module),
            Err(errors) => {
                diagnostics.extend(errors);
                None
            }
        };

        dbg!(&surface);
        // dbg!(&diagnostics);

        CompileOutput {
            parsed: Some(syntax),
            surface,
            diagnostics,
        }
    }
}
