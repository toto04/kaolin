pub mod image;

use crate::{
    commands::RenderCommand,
    kaolin::{Kaolin, scope::KaolinScope},
    renderers::KaolinRenderer,
    style::KaolinColor,
};
use embedded_graphics::{
    image::Image,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use u8g2_fonts::{
    FontRenderer,
    types::{FontColor, RenderedDimensions, VerticalPosition},
};

impl<C: PixelColor + Default> KaolinColor for C {}

/// A renderer that uses the embedded-graphics library to draw UI elements.
/// This renderer supports any color type that implements the `PixelColor` trait from embedded-graphics.
/// It also requires at least one font to be provided for text rendering, for now using [`u8g2_fonts`].
pub struct EmbeddedRenderer<Color>
where
    Color: PixelColor + Default + KaolinColor + 'static,
{
    fonts: &'static [FontRenderer],
    kaolin: Kaolin<Color>,
}

impl<Color> EmbeddedRenderer<Color>
where
    Color: PixelColor + Default + KaolinColor + 'static,
{
    pub fn new(fonts: &'static [FontRenderer], bounding_box: Rectangle) -> Self {
        let kaolin = Kaolin::new(
            (
                bounding_box.size.width as i32,
                bounding_box.size.height as i32,
            ),
            move |text, config| {
                let font = fonts.get(config.font_id as usize).unwrap_or(
                    fonts
                        .first()
                        .ok_or("At least one font must be provided")
                        .unwrap(),
                );
                let dimensions = font
                    .get_rendered_dimensions(text, (0, 0).into(), VerticalPosition::Top)
                    .unwrap_or_else(|_| RenderedDimensions::empty());
                match dimensions.bounding_box {
                    Some(box_) => (box_.size.width as f64, box_.size.height as f64),
                    None => (0.0, 0.0),
                }
            },
        );
        Self { fonts, kaolin }
    }

    /// Prepare to render onto a specific DrawTarget.
    /// This returns an intermediate structure that
    /// holds a mutable reference to the target, and implements [`KaolinRenderer`],
    /// onto which `draw` can be called.
    pub fn onto<'frame, D>(
        &'frame self,
        target: &'frame mut D,
    ) -> EmbeddedRendererFrame<'frame, D, Color>
    where
        D: DrawTarget<Color = Color>,
    {
        EmbeddedRendererFrame {
            renderer: self,
            target: Some(target),
        }
    }
}

pub struct EmbeddedRendererFrame<'frame, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + Default + KaolinColor + 'static,
{
    renderer: &'frame EmbeddedRenderer<C>,
    target: Option<&'frame mut D>,
}

impl<'frame, D, C, I> KaolinRenderer<'frame, C, Image<'frame, I>>
    for EmbeddedRendererFrame<'frame, D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor + Default + KaolinColor + 'static,
    I: ImageDrawable<Color = C> + Clone + Send + Sync + PartialEq,
{
    fn draw(
        &mut self,
        draw_fn: impl FnOnce(
            KaolinScope<'frame, C, Image<'frame, I>>,
        ) -> KaolinScope<'frame, C, Image<'frame, I>>,
    ) {
        let target = self.target.take().unwrap();
        let commands = self.renderer.kaolin.draw(draw_fn);
        // println!("Drawing {:?} commands", commands);
        for command in commands {
            match command {
                RenderCommand::DrawRectangle {
                    x,
                    y,
                    width,
                    height,
                    color,
                    border,
                    ..
                } => {
                    let _ = Rectangle::new(
                        Point::new(x as i32, y as i32),
                        Size::new(width as u32, height as u32),
                    )
                    .into_styled(
                        PrimitiveStyleBuilder::new()
                            .fill_color(color)
                            .stroke_color(border.color)
                            .stroke_width(border.width as u32)
                            .stroke_alignment(
                                embedded_graphics::primitives::StrokeAlignment::Inside,
                            )
                            .build(),
                    )
                    .draw(target);
                }
                RenderCommand::DrawText {
                    text,
                    x,
                    y,
                    font_id,
                    color,
                    ..
                } => {
                    let font = self
                        .renderer
                        .fonts
                        .get(font_id as usize)
                        .unwrap_or(self.renderer.fonts.first().unwrap());

                    let _ = font.render(
                        text.as_str(),
                        Point::new(x as i32, y as i32),
                        VerticalPosition::Top,
                        FontColor::Transparent(color),
                        target,
                    );
                }
                RenderCommand::Custom { x, y, data, .. } => {
                    let _ = data.translate(Point::new(x as i32, y as i32)).draw(target);
                }
            }
        }
    }
}
