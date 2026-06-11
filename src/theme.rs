use ratatui::style::Color;
use ratatui::style::palette::tailwind::{EMERALD, GRAY, GREEN, INDIGO, RED};

#[derive(Default)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub text: Color,
    pub error: Color,
    pub success: Color,
}

pub const THEME: Theme = Theme {
    primary: INDIGO.c600,
    secondary: EMERALD.c600,
    text: GRAY.c200,
    error: RED.c600,
    success: GREEN.c600,
};
