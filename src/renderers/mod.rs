use crate::kaolin::scope::KaolinScope;

pub trait KaolinRenderer<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    fn draw(&self, draw_fn: impl Fn(KaolinScope<Color>) -> KaolinScope<Color>);
}

#[cfg(feature = "raylib")]
pub mod raylib;
