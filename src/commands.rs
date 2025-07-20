use crate::style::colors::Color;

pub enum RenderCommand<'a> {
    DrawRectangle {
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
        font_size: u16,
        color: Color,
    },
}
