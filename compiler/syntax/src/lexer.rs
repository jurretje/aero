use crate::{
    source::{Location, Span},
    token::{Token, TokenKind},
};

#[derive(Debug, Clone)]
pub struct LexError {
    pub span: Span,
    pub character: char,
}

pub struct Lexer<'i> {
    input: &'i str,
    cursor: Location,
}

impl<'i> Lexer<'i> {
    pub fn new(input: &'i str) -> Self {
        Self {
            input,
            cursor: Location::default(),
        }
    }

    pub fn tokens(&mut self) -> (Vec<Token>, Vec<LexError>) {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        loop {
            match self.next() {
                Ok(Some(token)) => tokens.push(token),
                Ok(None) => break,
                Err(error) => errors.push(error),
            }
        }

        (tokens, errors)
    }

    fn skip_whitespace(&mut self) {
        let bytes = self.input.as_bytes();

        while let Some(&b) = bytes.get(self.cursor.position) {
            match b {
                b' ' | b'\t' | b'\r' => {
                    self.cursor.position += 1;
                    self.cursor.column += 1;
                }
                b'\n' => {
                    self.cursor.position += 1;
                    self.cursor.line += 1;
                    self.cursor.column = 0;
                }
                _ => break,
            }
        }
    }

    fn lex_token(&self, start: Location) -> Option<(Token, Location)> {
        let mut best = None;

        let slice = &self.input[start.position..];
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
        let bytes = &self.input.as_bytes()[start.position..start.position + len];
        for &b in bytes {
            if b == b'\n' {
                end.line += 1;
                end.column = 0;
            } else {
                end.column += 1;
            }
            end.position += 1;
        }

        end
    }

    pub fn next(&mut self) -> Result<Option<Token>, LexError> {
        self.skip_whitespace();

        if self.cursor.position >= self.input.len() {
            return Ok(None);
        }

        let start = self.cursor;

        match self.lex_token(start) {
            Some((token, end)) => {
                self.cursor = end;
                Ok(Some(token))
            }
            None => {
                let character = self.input[start.position..].chars().next().unwrap();
                let end = self.advance_location(start, character.len_utf8());

                self.cursor = end;
                Err(LexError {
                    span: Span { start, end },
                    character,
                })
            }
        }
    }
}
