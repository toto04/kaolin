use crate::{
    commands::RenderCommands,
    elements::flexbox::FlexBox,
    flex_style,
    style::{
        FlexStyle, TextConfig,
        sizing::{BoxSizing, Sizing},
    },
};

pub(crate) mod scope;

// #[]
pub(crate) type MeasureTextFn<'frame> = Box<dyn Fn(&str, &TextConfig) -> (f32, f32) + 'frame>;

pub struct Kaolin<'frame> {
    width: f32,
    height: f32,
    measure_text: MeasureTextFn<'frame>,
}

impl<'frame> Kaolin<'frame> {
    pub fn new(
        window_dimensions: (i32, i32),
        measure_text: impl Fn(&str, &TextConfig) -> (f32, f32) + 'frame,
    ) -> Self {
        let (width, height) = window_dimensions;
        let measure_text = Box::new(measure_text);
        Kaolin {
            width: width as f32,
            height: height as f32,
            measure_text,
        }
    }

    pub fn draw(
        &mut self,
        drawing_fn: fn(scope::KaolinScope) -> scope::KaolinScope,
    ) -> RenderCommands {
        let flex = FlexBox::new(flex_style! {
            sizing: BoxSizing {
                width: Sizing::Fixed(self.width),
                height: Sizing::Fixed(self.height),
            }
        });
        let mut scope = scope::KaolinScope::new(flex, &self.measure_text);
        scope = drawing_fn(scope);

        let mut flex = scope.conclude();
        flex.grow_children_width(self.width);
        flex.grow_children_height(self.height);
        flex.position_children((0.0, self.width, 0.0, self.height));
        flex.conclude()
    }
}
