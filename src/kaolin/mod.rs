use crate::{
    commands::RenderCommand,
    elements::flexbox::FlexBox,
    fixed,
    style::{
        FlexStyle, flex_style,
        sizing::{BoxSizing, Sizing},
    },
};

mod scope;

pub struct Kaolin {
    width: i32,
    height: i32,
    measure_text: fn(&str) -> (i32, i32),
}

impl Kaolin {
    pub fn new(window_dimensions: (i32, i32), measure_text: fn(&str) -> (i32, i32)) -> Self {
        let (width, height) = window_dimensions;
        Kaolin {
            width,
            height,
            measure_text,
        }
    }

    pub fn draw(
        &self,
        drawing_fn: fn(scope::KaolinScope<'_>) -> scope::KaolinScope<'_>,
    ) -> Vec<RenderCommand> {
        let mut flex = FlexBox::new(flex_style! {
            sizing: BoxSizing {
                width: fixed!(self.width as f32),
                height: fixed!(self.height as f32),
            }
        });
        let mut scope = scope::KaolinScope::new(flex);
        scope = drawing_fn(scope);

        flex = scope.extract();

        flex.grow_children_width(self.width as f32);
        // flex.

        Vec::new() // Here you would collect the commands from the scope
    }
}
