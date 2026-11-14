use elaborator::Elaborator;
use serde::{Deserialize, Serialize};
use source::{
    map::{SourceMap, SourceId},
    span::Span,
};
use surface::Module;
use syntax::{
    grammar::build_grammar,
    graph::{GrammarGraph, GrammarGraphBuilder},
    lexer::Lexer,
    node::Node,
    parser::{ParseCosts, build_ast, parse},
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

pub struct CompileOutput {}

pub struct Compiler {
    pub options: CompilerOptions,
    pub grammar_graph: GrammarGraph,
    pub sources: SourceMap,
}

impl Compiler {
    pub fn new() -> Self {
        let grammar_graph = GrammarGraphBuilder::new(&build_grammar()).build();

        Self {
            options: CompilerOptions::default(),
            grammar_graph,
            sources: SourceMap::default(),
        }
    }

    pub fn compile(&mut self, source_id: SourceId) -> CompileOutput {
        todo!()
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
