//! ## Renderers
//! the things that get the commands and draw stuff on the screen.

use crate::kaolin::scope::KaolinScope;

/// ### Renderer Trait
/// A renderer is responsible for taking a layout tree defined using Kaolin,
/// and converting it into actual drawing commands for a specific graphics backend.
/// This trait defines the interface that all renderers must implement.
pub trait KaolinRenderer<'frame, Color, CustomData>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    /// Draw the layout defined in the provided closure onto the target.
    /// The closure receives a `KaolinScope` which can be used to define the layout
    /// and add elements to it.
    /// The renderer should take the returned scope and conclude it, generating
    /// the commands via [`kaolin::commands::RenderCommands::new`].
    fn draw(
        &mut self,
        draw_fn: impl FnOnce(
            KaolinScope<'frame, Color, CustomData>,
        ) -> KaolinScope<'frame, Color, CustomData>,
    );
}

#[cfg(feature = "raylib")]
pub mod raylib;

#[cfg(not(feature = "raylib"))]
#[cfg(feature = "embedded")]
pub mod embedded;
