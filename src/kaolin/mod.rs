use crate::{
    commands::RenderCommand,
    elements::{flexbox::FlexBox, text::TextConfig},
    fixed,
    style::{
        FlexStyle, flex_style,
        sizing::{BoxSizing, Sizing},
    },
};

mod scope;

pub type MeasureTextFn = fn(&str, &TextConfig) -> (f32, f32);

pub struct Kaolin {
    width: f32,
    height: f32,
    measure_text: MeasureTextFn,
    root_ref: Option<FlexBox>,
}

impl Kaolin {
    pub fn new(window_dimensions: (i32, i32), measure_text: MeasureTextFn) -> Self {
        let (width, height) = window_dimensions;
        Kaolin {
            width: width as f32,
            height: height as f32,
            measure_text,
            root_ref: None,
        }
    }

    pub fn draw(
        &mut self,
        drawing_fn: fn(scope::KaolinScope) -> scope::KaolinScope,
    ) -> Vec<RenderCommand> {
        let flex = FlexBox::new(flex_style! {
            sizing: BoxSizing {
                width: fixed!(self.width),
                height: fixed!(self.height),
            }
        });
        let mut scope = scope::KaolinScope::new(flex, self.measure_text);
        scope = drawing_fn(scope);

        self.root_ref = Some(scope.extract());
        let flex = self.root_ref.as_mut().unwrap();
        flex.grow_children_width(self.width);
        flex.grow_children_height(self.height);
        flex.position_children((0.0, self.width, 0.0, self.height));

        let commands = Vec::new();
        flex.render_all(commands)
    }
}
