pub mod layout;
pub mod padding;
pub mod sizing;

use derive_default_constructor::DefaultConstructor;
use derive_setters::Setters;

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

#[derive(DefaultConstructor, Clone, Copy, Setters)]
pub struct FlexStyle<Color>
where
    Color: std::default::Default + Copy + PartialEq + KaolinColor<Color>,
{
    /// TODO: Doesn't do anything right now
    pub color: Color,
    /// The background color for the flex item
    pub background_color: Color,
    /// The layout configuration for the children of the flex item
    pub layout: Layout,
    /// The sizing configuration for the flex item
    pub sizing: BoxSizing,
    /// The padding around the flex item
    pub padding: Padding,
    /// The corner radius for the flex item
    pub corner_radius: f32,
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

#[derive(DefaultConstructor, Debug, Clone, Copy, PartialEq, Setters)]
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
