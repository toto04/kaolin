pub mod border;
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
    /// default color for things like text and borders.
    fn default_foreground_color() -> Color {
        Color::default()
    }
    /// default color for background for flex containers.
    fn default_background_color() -> Color {
        Color::default()
    }
}

/// Style for a flex container
#[derive(DefaultConstructor, Clone, Copy, Setters)]
pub struct FlexStyle<Color>
where
    Color: std::default::Default + Copy + PartialEq + KaolinColor<Color>,
{
    /// Set the text color to be inherited by default by all child elements
    #[setters(strip_option)]
    pub color: Option<Color>,
    /// The background color for the flex item
    #[setters(strip_option)]
    pub background_color: Option<Color>,
    /// The layout configuration for the children of the flex item
    pub layout: Layout,
    /// The sizing configuration for the flex item
    pub sizing: BoxSizing,
    /// The padding around the flex item
    pub padding: Padding,
    /// The corner radius for the flex item
    pub corner_radius: f32,
    /// The border configuration for the flex item
    pub border: border::Border<Color>,
}

impl<Color> Default for FlexStyle<Color>
where
    Color: Default + Copy + PartialEq + KaolinColor<Color>,
{
    fn default() -> Self {
        FlexStyle {
            color: None,
            background_color: None,
            layout: Layout::default(),
            sizing: BoxSizing::default(),
            padding: Padding::default(),
            corner_radius: 0.0,
            border: border::Border::default(),
        }
    }
}

/// Style for a text element.
#[derive(DefaultConstructor, Debug, Clone, Copy, PartialEq, Setters)]
pub struct TextStyle<Color>
where
    Color: Default + Copy + PartialEq + KaolinColor<Color>,
{
    /// The ID of the font to use for the text element. Intepretation depends on the renderer.
    pub font_id: u32,
    /// The size of the font to use for the text element. (Default: 16.0)
    pub font_size: f32,
    /// Color of the text.
    #[setters(strip_option)]
    pub color: Option<Color>,
}

impl<Color> Default for TextStyle<Color>
where
    Color: Default + Copy + PartialEq + KaolinColor<Color>,
{
    fn default() -> Self {
        TextStyle {
            font_id: 0,
            font_size: 16.0,
            color: None,
        }
    }
}
