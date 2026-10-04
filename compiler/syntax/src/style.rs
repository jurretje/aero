use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Style {
    Keyword,
    Type,
    Identifier,
    Operator,
    Punctuation,
}

impl Style {
    pub fn ansi(self) -> &'static str {
        let default = match self {
            Style::Keyword => "\x1b[38;2;255;121;198m",
            Style::Type => "\x1b[38;2;139;233;253m",
            Style::Identifier => "\x1b[38;2;220;220;220m",
            Style::Operator => "\x1b[38;2;255;184;108m",
            Style::Punctuation => "\x1b[38;2;98;114;164m",
        };
        default
    }
}
