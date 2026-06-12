use ratatui::style::Color;
use ratatui::style::palette::tailwind::{BLUE, EMERALD, GRAY, GREEN, INDIGO, RED};

use crate::api::RequestType;

#[derive(Default)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub text: Color,
    pub background: Color,
    pub error: Color,
    pub success: Color,

    // Requests Type Colors
    pub get: Color,
    pub post: Color,
    pub delete: Color,
    pub patch: Color,
    pub put: Color,
}

impl Theme {
    pub fn derive_request(&self, request: RequestType) -> Color {
        match request {
            RequestType::GET => self.get,
            RequestType::POST => self.post,
            RequestType::DELETE => self.delete,
            RequestType::PATCH => self.patch,
            RequestType::PUT => self.put,
        }
    }

    pub fn derive_body(&self, request: RequestType) -> Color {
        match request {
            RequestType::GET | RequestType::DELETE => self.background,
            RequestType::PUT | RequestType::PATCH | RequestType::POST => self.text,
        }
    }
}

pub const THEME: Theme = Theme {
    primary: INDIGO.c600,
    secondary: EMERALD.c600,
    text: GRAY.c200,
    background: GRAY.c500,
    error: RED.c600,
    success: GREEN.c600,

    get: BLUE.c300,
    post: EMERALD.c300,
    delete: RED.c300,
    patch: INDIGO.c300,
    put: GREEN.c300,
};
