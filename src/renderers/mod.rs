use crate::kaolin::scope::KaolinScope;

pub mod raylib;

pub trait KaolinRenderer {
    fn draw(&mut self, draw_fn: fn(KaolinScope) -> KaolinScope);
}
