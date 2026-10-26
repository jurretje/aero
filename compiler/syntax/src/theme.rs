use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn ansi(self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Style {
    Keyword,
    Type,
    Identifier,
    Operator,
    Punctuation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThemeName {
    #[default]
    Feather,
    Monokai,
    TokyoNight,
}

impl ThemeName {
    pub fn build(self) -> Theme {
        match self {
            ThemeName::Feather => Theme::feather(),
            ThemeName::Monokai => Theme::monokai(),
            ThemeName::TokyoNight => Theme::tokyo_night(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub keyword: Color,
    pub type_name: Color,
    pub identifier: Color,
    pub operator: Color,
    pub punctuation: Color,
}

impl Theme {
    pub fn feather() -> Self {
        Self {
            keyword: Color::rgb(255, 121, 198),
            type_name: Color::rgb(139, 233, 253),
            identifier: Color::rgb(220, 220, 220),
            operator: Color::rgb(255, 184, 108),
            punctuation: Color::rgb(98, 114, 164),
        }
    }

    pub fn monokai() -> Self {
        Self {
            keyword: Color::rgb(249, 38, 114),
            type_name: Color::rgb(166, 226, 46),
            identifier: Color::rgb(248, 248, 242),
            operator: Color::rgb(249, 38, 114),
            punctuation: Color::rgb(144, 136, 119),
        }
    }

    pub fn tokyo_night() -> Self {
        Self {
            keyword: Color::rgb(187, 154, 247),
            type_name: Color::rgb(125, 207, 255),
            identifier: Color::rgb(192, 202, 245),
            operator: Color::rgb(137, 221, 255),
            punctuation: Color::rgb(86, 95, 137),
        }
    }

    pub fn color(&self, style: Style) -> Color {
        match style {
            Style::Keyword => self.keyword,
            Style::Type => self.type_name,
            Style::Identifier => self.identifier,
            Style::Operator => self.operator,
            Style::Punctuation => self.punctuation,
        }
    }
}
