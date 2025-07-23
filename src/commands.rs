use crate::{elements::text::TextConfig, style::colors::Color};

#[derive(Debug, Clone, Copy)]
pub enum RenderCommand<'a> {
    DrawRectangle {
        id: &'a str,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: Color,
    },
    DrawText {
        text: &'a str,
        x: i32,
        y: i32,
        config: &'a TextConfig,
    },
}
