pub mod layout;
pub mod padding;
pub mod sizing;

use crate::style::layout::Layout;
use crate::style::padding::Padding;
use crate::style::sizing::BoxSizing;

pub trait KaolinColor<Color>
where
    Color: Default + Copy + PartialEq,
{
    fn default_foreground_color() -> Color {
        Color::default()
    }
    fn default_background_color() -> Color {
        Color::default()
    }
}

pub struct FlexStyle<Color>
where
    Color: Default + Copy + PartialEq + KaolinColor<Color>,
{
    pub color: Color,
    pub background_color: Color,
    pub layout: Layout,
    pub sizing: BoxSizing,
    pub padding: Padding,   // Padding around the flex item
    pub corner_radius: f32, // Optional corner radius for rounded corners
}

impl<Color> Default for FlexStyle<Color>
where
    Color: Default + Copy + PartialEq + KaolinColor<Color>,
{
    fn default() -> Self {
        FlexStyle {
            color: Color::default_foreground_color(),
            background_color: Color::default_background_color(),
            layout: Layout::default(),
            sizing: BoxSizing::default(),
            padding: Padding::default(),
            corner_radius: 0.0,
        }
    }
}

#[macro_export]
macro_rules! flex_style {
    () => {
        FlexStyle::default()
    };
    ($($key:ident : $value:expr),* $(,)?) => {
        FlexStyle {
            $( $key: $value, )*
            ..FlexStyle::default()
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextConfig<Color>
where
    Color: Default + Copy + PartialEq + KaolinColor<Color>,
{
    pub font_id: u32,
    pub font_size: f32,
    pub color: Option<Color>,
}

impl<Color> Default for TextConfig<Color>
where
    Color: Default + Copy + PartialEq + KaolinColor<Color>,
{
    fn default() -> Self {
        TextConfig {
            font_id: 0,
            font_size: 16.0,
            color: None,
        }
    }
}

#[macro_export]
macro_rules! text_config {
    () => {
        kaolin::style::TextConfig::default()
    };
    ($($key:ident : $value:expr),* $(,)?) => {
        kaolin::style::TextConfig {
            $( $key: $value, )*
            ..kaolin::style::TextConfig::default()
        }
    };
}
