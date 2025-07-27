pub mod colors;
pub mod layout;
pub mod padding;
pub mod sizing;

use crate::style::colors::*;
use crate::style::layout::Layout;
use crate::style::padding::Padding;
use crate::style::sizing::BoxSizing;

pub struct FlexStyle {
    pub color: Color,
    pub background_color: Color,
    pub layout: Layout,
    pub sizing: BoxSizing,
    pub padding: Padding,   // Padding around the flex item
    pub corner_radius: f32, // Optional corner radius for rounded corners
}

impl Default for FlexStyle {
    fn default() -> Self {
        FlexStyle {
            color: Colors::Black.into(),
            background_color: Colors::Transparent.into(),
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
pub struct TextConfig {
    pub font_id: u32,
    pub font_size: f32,
    pub color: Color,
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
