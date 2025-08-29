//! ## Renderers
//! the things that get the commands and draw stuff on the screen.

use crate::kaolin::scope::KaolinScope;

pub trait KaolinRenderer<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    fn draw(&mut self, draw_fn: impl Fn(KaolinScope<Color>) -> KaolinScope<Color>);
}

#[cfg(feature = "raylib")]
pub mod raylib;

#[cfg(not(feature = "raylib"))]
#[cfg(feature = "embedded")]
pub mod embedded;
