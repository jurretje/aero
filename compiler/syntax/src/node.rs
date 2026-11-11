use serde::Serialize;

use crate::{
    grammar::RuleName,
    source::Span,
    theme::{Style, Theme},
    token::TokenKind,
};

#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
pub enum NodeKind {
    Token(TokenKind),
    Rule(RuleName),
}

#[derive(Debug, Serialize, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub span: Span,
    pub children: Vec<Node>,
}

impl Node {
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

    pub fn print_tree(&self, input: &str, theme: &Theme) {
        println!("{:?}", self.kind);
        self.write_tree(input, "", theme);
    }

    fn label(&self, input: &str) -> String {
        match self.kind {
            NodeKind::Token(kind) => kind.literal().unwrap_or(self.span.text(input)).to_string(),
            NodeKind::Rule(name) => {
                format!("{:?}", name)
            }
        }
    }

    fn colored_label(&self, input: &str, theme: &Theme) -> String {
        let (label, style) = match self.kind {
            NodeKind::Token(kind) => (self.label(input), kind.style()),
            NodeKind::Rule(_) => (self.label(input), Style::Punctuation),
        };

        format!("{}{}{}", theme.color(style).ansi(), label, "\x1b[0m")
    }

    fn write_tree(&self, input: &str, prefix: &str, theme: &Theme) {
        for (i, child) in self.children.iter().enumerate() {
            let is_last = i == self.children.len() - 1;

            let branch = if is_last { "└── " } else { "├── " };

            println!("{}{}{}", prefix, branch, child.colored_label(input, theme));

            let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

            child.write_tree(input, &child_prefix, theme);
        }
    }
}
