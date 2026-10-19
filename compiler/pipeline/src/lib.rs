use serde::Serialize;
use syntax::{
    grammar::build_grammar,
    graph::GrammarGraphBuilder,
    lexer::Lexer,
    node::Node,
    parser::{ParseCosts, build_ast, parse},
    source::Span,
    style::Style,
    token::TokenKind,
};

pub struct Compiler {
    parse_costs: ParseCosts,
}

#[derive(Debug, Serialize)]
pub struct StylizedToken {
    kind: TokenKind,
    span: Span,
    style: Style,
}

pub struct Analysis {}

pub struct CompileOutput {
    pub parsed: Option<Node>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            parse_costs: ParseCosts {
                insert: 2,
                delete: 4,
                replace: 5,
            },
        }
    }

    pub fn analyze(&self, source: &str) -> Analysis {
        todo!()
    }

    pub fn compile(&self, source: &str) -> CompileOutput {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokens();

        let grammar = build_grammar();
        let grammar_graph = GrammarGraphBuilder::new(&grammar).build();

        let parsed = parse(&grammar_graph, &tokens, self.parse_costs)
            .and_then(|actions| build_ast(&actions, &tokens));

        CompileOutput { parsed }
    }
}
