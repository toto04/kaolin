use crate::style::colors::Color;

pub struct TextConfig {
    font_id: u32,
    font_size: f32,
    color: Color,
}

impl Default for TextConfig {
    fn default() -> Self {
        TextConfig {
            font_id: 0,
            font_size: 16.0,
            color: Color::default(),
        }
    }
}

pub struct TextElement<'a> {
    content: String,
    lines: Vec<&'a str>,
    config: TextConfig,
}

impl<'a> TextElement<'a> {
    pub fn new(content: &str, config: TextConfig) -> Self {
        let lines = Vec::new();
        TextElement {
            content: content.to_string(),
            lines,
            config,
        }
    }

    pub(crate) fn wrap_text(&self, current_width: f32) {
        todo!()
    }
}
