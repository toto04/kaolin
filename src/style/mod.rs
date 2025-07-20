pub mod colors;
pub mod layout;
pub mod padding;
pub mod sizing;

use derive_declare::Declare;

use crate::style::colors::*;
use crate::style::layout::Layout;
use crate::style::padding::Padding;
use crate::style::sizing::BoxSizing;

#[derive(Declare)]
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
