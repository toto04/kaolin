//! ## Commands
//! Definitions of the rendering commands that will be used to draw the UI elements.
//! Those should be used as a reference for implementing custom renderers.

use alloc::{boxed::Box, string::String};
use ouroboros::self_referencing;

use crate::{elements::flexbox::FlexBox, style::border, utils::floats::Float};

/// A single rendering command.
#[derive(Debug, Clone)]
pub enum RenderCommand<Color, CustomData = !>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    /// Draws a rectangle on the screen.
    DrawRectangle {
        /// The absolute x position of the rectangle.
        x: Float,
        /// The absolute y position of the rectangle.
        y: Float,
        /// The width of the rectangle.
        width: Float,
        /// The height of the rectangle.
        height: Float,
        /// The background color of the rectangle.
        color: Color,
        /// The corner radius of the rectangle.
        corner_radius: Float,
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
        x: Float,
        /// The absolute y position of the text.
        y: Float,
        /// The font ID of the text, passed through from the style.
        /// Interpretation of this value depends on the renderer.
        font_id: u32,
        /// The font size of the text.
        font_size: Float,
        /// The color of the text.
        color: Color,
    },

    /// A custom render command, which carries arbitrary data for the renderer to interpret.
    Custom {
        /// The absolute x position of the custom element.
        x: Float,
        /// The absolute y position of the custom element.
        y: Float,
        /// The width of the custom element.
        width: Float,
        /// The height of the custom element.
        height: Float,
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

#[self_referencing]
struct RenderCommandsInner<'a, Color, CustomData>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor + 'a,
    CustomData: 'a,
{
    root: FlexBox<'a, Color, CustomData>,
    #[borrows(root)]
    #[covariant]
    iter: Box<dyn Iterator<Item = RenderCommand<Color, CustomData>> + 'this>,
}

/// Represents a series of rendering commands.
///
/// This struct implements an iterator of the render commands, which should be processed in order.
pub struct RenderCommands<'a, Color, CustomData>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    inner: RenderCommandsInner<'a, Color, CustomData>,
}

impl<'a, Color, CustomData> RenderCommands<'a, Color, CustomData>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    /// Creates a new set of render commands from a root layout.
    pub(crate) fn new(root: FlexBox<'a, Color, CustomData>) -> Self {
        RenderCommands {
            inner: RenderCommandsInnerBuilder {
                root,
                iter_builder: |root| Box::new(root.children.render_nodes()),
            }
            .build(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.borrow_iter().size_hint().0 == 0
    }

    pub fn len(&self) -> usize {
        self.inner.borrow_iter().size_hint().0
    }
}

impl<'a, Color, CustomData> Iterator for RenderCommands<'a, Color, CustomData>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    type Item = RenderCommand<Color, CustomData>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.with_iter_mut(|iter| iter.next())
    }
}
