use alloc::{
    boxed::Box,
    rc::{Rc, Weak},
};

use crate::{
    commands::RenderCommands,
    elements::flexbox::FlexBox,
    fixed, sizing,
    style::{FlexStyle, TextStyle},
};

pub mod scope;

pub type MeasureTextFnStatic<Color> = Box<dyn Fn(&str, &TextStyle<Color>) -> (f64, f64)>;
pub(crate) type MeasureTextFnRef<Color> = Weak<MeasureTextFnStatic<Color>>;

pub struct Kaolin<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color> + 'static,
{
    width: f64,
    height: f64,
    measure_text: Rc<MeasureTextFnStatic<Color>>,
}

impl<Color> Kaolin<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color> + 'static,
{
    /// Creates a new instance of Kaolin with the specified window dimensions and text measurement function.
    pub fn new(
        window_dimensions: (i32, i32),
        measure_text: impl Fn(&str, &TextStyle<Color>) -> (f64, f64) + 'static,
    ) -> Self {
        let (width, height) = window_dimensions;
        let measure_text: Rc<MeasureTextFnStatic<Color>> = Rc::new(Box::new(measure_text));
        Kaolin {
            width: width as f64,
            height: height as f64,
            measure_text,
        }
    }

    pub fn draw(
        &self,
        drawing_fn: impl Fn(scope::KaolinScope<Color>) -> scope::KaolinScope<Color>,
    ) -> RenderCommands<Color> {
        let flex = FlexBox::new(FlexStyle::default().sizing(sizing! {
            width: fixed!(self.width),
            height: fixed!(self.height),
        }));
        let measure_text_weak: MeasureTextFnRef<Color> = Rc::downgrade(&self.measure_text);
        let mut scope = scope::KaolinScope::new(flex, measure_text_weak);
        scope = drawing_fn(scope);

        let mut flex = scope.conclude();
        flex.grow_children_width(self.width);
        flex.grow_children_height(self.height);
        flex.position_children((0.0, self.width, 0.0, self.height));
        flex.conclude()
    }
}
