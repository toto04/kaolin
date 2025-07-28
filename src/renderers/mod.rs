use crate::kaolin::scope::KaolinScope;

pub trait KaolinRenderer {
    fn draw(&mut self, draw_fn: fn(KaolinScope) -> KaolinScope);
}

#[cfg(feature = "raylib")]
pub mod raylib;
