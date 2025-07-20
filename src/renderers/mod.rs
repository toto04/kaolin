pub mod raylib;

pub trait KaolinRenderer {
    fn draw(&mut self, commands: Vec<crate::commands::RenderCommand>);
}
