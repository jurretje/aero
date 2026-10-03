use crate::{
    source::{Location, Span},
    token::{Token, TokenKind},
};

pub struct Lexer<'i> {
    input: &'i str,
    cursor: Location,
}

impl<'i> Lexer<'i> {
    pub fn new(input: &'i str) -> Self {
        Self {
            input,
            cursor: Location {
                pos: 0,
                line: 0,
                column: 0,
            },
        }
    }

    pub fn tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next() {
            tokens.push(token);
        }

        tokens
    }

    fn skip_whitespace(&mut self) {
        let bytes = self.input.as_bytes();

        while let Some(&b) = bytes.get(self.cursor.pos) {
            match b {
                b' ' | b'\t' | b'\r' => {
                    self.cursor.pos += 1;
                    self.cursor.column += 1;
                }
                b'\n' => {
                    self.cursor.pos += 1;
                    self.cursor.line += 1;
                    self.cursor.column = 0;
                }
                _ => break,
            }
        }
    }

    fn lex_token(&self, start: Location) -> Option<(Token, Location)> {
        let mut best = None;

        let slice = &self.input[start.pos..];
        for kind in TokenKind::all() {
            if let Some((k, len)) = kind.matches(slice) {
                match best {
                    Some((_, best_len)) if len > best_len => best = Some((k, len)),
                    None => best = Some((k, len)),
                    _ => {}
                }
            }
        }

        let (kind, len) = best?;

        let end = self.advance_location(start, len);

        Some((
            Token {
                kind,
                span: Span { start, end },
            },
            end,
        ))
    }

    fn advance_location(&self, start: Location, len: usize) -> Location {
        let mut end = start;
        let bytes = &self.input.as_bytes()[start.pos..start.pos + len];
        for &b in bytes {
            if b == b'\n' {
                end.line += 1;
                end.column = 0;
            } else {
                end.column += 1;
            }
            end.pos += 1;
        }

        end
    }

    pub fn next(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let (tok, end) = self.lex_token(self.cursor)?;

        self.cursor = end;

        Some(tok)
    }
}
