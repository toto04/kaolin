use core::iter;

use alloc::{boxed::Box, string::ToString};
use embedded_graphics::{
    image::{Image, ImageDrawable},
    prelude::{Dimensions, PixelColor},
};
use typed_floats::tf64::PositiveFinite;

use crate::{
    commands::RenderCommand,
    elements::KaolinElement,
    style::{KaolinColor, sizing},
};

/// Implementation of KaolinElement for embedded-graphics Image
///
/// This allows you to use embedded-graphics images as elements within a Kaolin layout.
/// See [`KaolinScope::with_element`] for more details.
impl<'frame, Color, I> KaolinElement<'frame, Color, Image<'frame, I>> for Image<'frame, I>
where
    Color: PixelColor + Default + KaolinColor + From<<Color as PixelColor>::Raw> + 'static,
    I: ImageDrawable<Color = Color> + Clone + 'frame,
{
    fn get_sizing_dimensions(
        &self,
    ) -> (
        crate::style::sizing::SizingDimensions,
        crate::style::sizing::SizingDimensions,
    ) {
        let raw_size = self.bounding_box().size;
        (
            sizing::Sizing::Fixed(PositiveFinite::from(raw_size.width)).into(),
            sizing::Sizing::Fixed(PositiveFinite::from(raw_size.height)).into(),
        )
    }

    fn render(
        &self,
        offsets: (f64, f64),
        size: (f64, f64),
    ) -> alloc::boxed::Box<
        dyn Iterator<Item = crate::commands::RenderCommand<Color, Image<'frame, I>>> + '_,
    > {
        Box::new(iter::once(RenderCommand::Custom {
            id: "".to_string(),
            x: offsets.0,
            y: offsets.1,
            width: size.0,
            height: size.1,
            data: self.clone(),
        }))
    }

    fn as_container(
        &mut self,
    ) -> Option<&mut dyn crate::elements::KaolinContainerElement<'frame, Color, Image<'frame, I>>>
    {
        None
    }
}
