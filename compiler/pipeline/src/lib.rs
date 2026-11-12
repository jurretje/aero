use elaborator::Elaborator;
use serde::{Deserialize, Serialize};
use surface::Module;
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

use crate::{diagnostics::Diagnostic, lowering::Lowerer};

pub mod diagnostics;
pub mod lowering;

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
    pub surface: Option<Module>,
    pub diagnostics: Vec<Diagnostic>,
}

pub struct Compiler {
    pub options: CompilerOptions,
    pub grammar_graph: GrammarGraph,
}

impl Compiler {
    pub fn new() -> Self {
        let grammar_graph = GrammarGraphBuilder::new(&build_grammar()).build();

        Self {
            options: CompilerOptions::default(),
            grammar_graph,
        }
    }

    pub fn compile(&self, source: &str) -> CompileOutput {
        let mut lexer = Lexer::new(source);
        let (tokens, _) = lexer.tokens();

        // dbg!(&tokens);

        let parser_output = parse(&self.grammar_graph, &tokens, self.options.parse_costs);
        let actions = parser_output.actions;

        let edits = actions.iter().filter(|a| a.is_edit()).collect::<Vec<_>>();

        if !edits.is_empty() {
            dbg!(edits);
        }

        let syntax = build_ast(&actions, &tokens);

        let Some(syntax) = syntax else {
            return CompileOutput {
                parsed: None,
                surface: None,
                diagnostics: vec![],
            };
        };

        syntax.print_tree(source, &self.options.theme.build());

        let mut diagnostics = Vec::new();

        let mut lowerer = Lowerer::default();

        let surface = match lowerer.lower(syntax.clone()) {
            Ok(module) => module,
            Err(errors) => {
                diagnostics.extend(errors);
                return CompileOutput {
                    parsed: Some(syntax),
                    surface: None,
                    diagnostics,
                };
            }
        };

        dbg!(&surface);

        let mut elaborator = Elaborator::new();

        let core = match elaborator.elaborate_module(&surface) {
            Ok(module) => Some(module),
            Err(errors) => {
                diagnostics.extend(errors.into_iter().map(|e| Diagnostic::ElabError(e)));

                return CompileOutput {
                    parsed: Some(syntax),
                    surface: Some(surface),
                    diagnostics,
                };
            }
        };

        // dbg!(&core);

        CompileOutput {
            parsed: Some(syntax),
            surface: Some(surface),
            diagnostics,
        }
    }

    pub fn analyze(&self, source: &str) -> Analysis {
        let mut lexer = Lexer::new(source);
        let (tokens, _) = lexer.tokens();

        let parser_output = parse(&self.grammar_graph, &tokens, self.options.parse_costs);

        let ast = build_ast(&parser_output.actions, &tokens);

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
}
