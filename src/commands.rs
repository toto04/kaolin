//! ## Commands
//! Definitions of the rendering commands that will be used to draw the UI elements.
//! Those should be used as a reference for implementing custom renderers.

use alloc::{collections::VecDeque, string::String};

use crate::{elements::flexbox::FlexBox, style::border};

/// A single rendering command.
#[derive(Debug, Clone)]
pub enum RenderCommand<Color, CustomData = !>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    /// Draws a rectangle on the screen.
    DrawRectangle {
        id: String,
        /// The absolute x position of the rectangle.
        x: f64,
        /// The absolute y position of the rectangle.
        y: f64,
        /// The width of the rectangle.
        width: f64,
        /// The height of the rectangle.
        height: f64,
        /// The background color of the rectangle.
        color: Color,
        /// The corner radius of the rectangle.
        corner_radius: f32,
        /// The border settings of the rectangle (width, color).
        border: border::Border<Color>,
    },
    /// Draws text on the screen.
    ///
    /// Multiple DrawText commands can be issued to draw a single text element
    /// on different lines.
    DrawText {
        /// The text to be drawn, in a single line. This is only converted to a
        /// String when generating the specific command to allow for safe deallocation
        /// of the input string within the [`KaolinScope::text`] call. Internally,
        /// calculations are performed with `&str` slices.
        text: String,
        /// The absolute x position of the text.  
        x: f64,
        /// The absolute y position of the text.
        y: f64,
        /// The font ID of the text, passed through from the style.
        /// Interpretation of this value depends on the renderer.
        font_id: u32,
        /// The font size of the text.
        font_size: f32,
        /// The color of the text.
        color: Color,
    },

    /// A custom render command, which carries arbitrary data for the renderer to interpret.
    Custom {
        /// not implemented yet
        id: String,
        /// The absolute x position of the custom element.
        x: f64,
        /// The absolute y position of the custom element.
        y: f64,
        /// The width of the custom element.
        width: f64,
        /// The height of the custom element.
        height: f64,
        /// The custom data associated with the element.
        data: CustomData,
    },
}

impl<Color, CustomData> PartialEq for RenderCommand<Color, CustomData>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                RenderCommand::DrawRectangle {
                    x,
                    y,
                    width,
                    height,
                    color,
                    corner_radius,
                    ..
                },
                RenderCommand::DrawRectangle {
                    x: other_x,
                    y: other_y,
                    width: other_width,
                    height: other_height,
                    color: other_color,
                    corner_radius: other_corner_radius,
                    ..
                },
            ) => {
                x == other_x
                    && y == other_y
                    && width == other_width
                    && height == other_height
                    && color == other_color
                    && corner_radius == other_corner_radius
            }
            (
                RenderCommand::DrawText {
                    text,
                    x,
                    y,
                    font_id,
                    font_size,
                    color,
                },
                RenderCommand::DrawText {
                    text: other_text,
                    x: other_x,
                    y: other_y,
                    font_id: other_font_id,
                    font_size: other_font_size,
                    color: other_color,
                },
            ) => {
                text == other_text
                    && x == other_x
                    && y == other_y
                    && font_id == other_font_id
                    && font_size == other_font_size
                    && color == other_color
            }
            _ => false,
        }
    }
}

/// Represents a series of rendering commands.
///
/// This struct implements an iterator of the render commands, which should be processed in order.
#[derive(Debug, Clone, PartialEq)]
pub struct RenderCommands<Color, CustomData>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    commands: VecDeque<RenderCommand<Color, CustomData>>,
}

impl<Color, CustomData> RenderCommands<Color, CustomData>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    /// Creates a new set of render commands from a root layout.
    pub(crate) fn new(root: FlexBox<Color, CustomData>) -> Self {
        let children = root.children;
        RenderCommands {
            commands: children.render_nodes().collect::<VecDeque<_>>(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn len(&self) -> usize {
        self.commands.len()
    }
}

impl<Color, CustomData> Iterator for RenderCommands<Color, CustomData>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    type Item = RenderCommand<Color, CustomData>;

    fn next(&mut self) -> Option<Self::Item> {
        self.commands.pop_front()
    }
}
