use serde::Serialize;

use crate::{grammar::RuleName, source::Span, theme::Theme, token::TokenKind};

#[derive(Debug, Serialize)]
pub enum NodeKind {
    Token(TokenKind),
    Rule(RuleName),
}

#[derive(Debug, Serialize)]
pub struct Node {
    pub kind: NodeKind,
    pub span: Span,
    pub children: Vec<Node>,
}

impl Node {
    pub fn dump(&self, indent: usize) {
        let padding = " ".repeat(indent);

        println!("{padding}{:?}", self.kind);

        for child in &self.children {
            child.dump(indent + 1);
        }
    }

    pub fn highlighted(&self, input: &str, theme: &Theme) -> String {
        let mut out = String::new();
        self.write_highlighted(input, &mut out, theme);
        out
    }

    fn write_highlighted(&self, input: &str, out: &mut String, theme: &Theme) {
        match self.kind {
            NodeKind::Token(kind) => {
                out.push_str(&theme.color(kind.style()).ansi());
                out.push_str(self.span.text(input));
                out.push_str("\x1b[0m");
            }

            NodeKind::Rule(_) => {
                let mut cursor = self.span.start.pos;

                for child in &self.children {
                    if cursor < child.span.start.pos {
                        out.push_str(&input[cursor..child.span.start.pos]);
                    }

                    child.write_highlighted(input, out, theme);

                    cursor = child.span.end.pos;
                }

                if cursor < self.span.end.pos {
                    out.push_str(&input[cursor..self.span.end.pos]);
                }
            }
        }
    }
}
