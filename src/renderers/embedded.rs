use crate::{
    commands::RenderCommand,
    kaolin::{Kaolin, scope::KaolinScope},
    renderers::KaolinRenderer,
    style::KaolinColor,
};
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};
use u8g2_fonts::{
    FontRenderer,
    types::{FontColor, RenderedDimensions, VerticalPosition},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct KBinaryColor {
    color: BinaryColor,
}

impl From<BinaryColor> for KBinaryColor {
    fn from(color: BinaryColor) -> Self {
        Self { color }
    }
}
impl From<KBinaryColor> for BinaryColor {
    fn from(k_color: KBinaryColor) -> Self {
        k_color.color
    }
}

impl KaolinColor<KBinaryColor> for KBinaryColor {
    fn default_foreground_color() -> KBinaryColor {
        KBinaryColor::from(BinaryColor::On)
    }

    fn default_background_color() -> KBinaryColor {
        KBinaryColor::from(BinaryColor::Off)
    }
}

pub struct EmbeddedRenderer {
    fonts: &'static [FontRenderer],
    kaolin: Kaolin<KBinaryColor>,
}

impl EmbeddedRenderer {
    pub fn new(fonts: &'static [FontRenderer], bounding_box: Rectangle) -> Self {
        let kaolin = Kaolin::new(
            (
                bounding_box.size.width as i32,
                bounding_box.size.height as i32,
            ),
            move |text, config| {
                let font = fonts
                    .get(config.font_id as usize)
                    .unwrap_or(fonts.first().unwrap());
                let dimensions = font
                    .get_rendered_dimensions(text, (0, 0).into(), VerticalPosition::Top)
                    .unwrap_or_else(|_| RenderedDimensions::empty());
                match dimensions.bounding_box {
                    Some(box_) => (box_.size.width as f64, box_.size.height as f64),
                    None => (0.0, 0.0),
                }
                // (2.0, 2.0) // Placeholder for actual dimensions
            },
        );
        Self { fonts, kaolin }
    }
}

impl EmbeddedRenderer {
    pub fn onto<'frame, D, C>(
        &'frame self,
        target: &'frame mut D,
    ) -> EmbeddedRendererFrame<'frame, D, C>
    where
        D: DrawTarget<Color = C>,
        C: Into<KBinaryColor>,
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
{
    renderer: &'frame EmbeddedRenderer,
    target: Option<&'frame mut D>,
}

impl<'frame, D> KaolinRenderer<KBinaryColor> for EmbeddedRendererFrame<'frame, D, BinaryColor>
where
    D: DrawTarget<Color = BinaryColor>,
{
    fn draw(&mut self, draw_fn: impl Fn(KaolinScope<KBinaryColor>) -> KaolinScope<KBinaryColor>) {
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
                    ..
                } => {
                    let color: BinaryColor = color.into();
                    let _ = Rectangle::new(
                        Point::new(x as i32, y as i32),
                        Size::new(width as u32, height as u32),
                    )
                    .into_styled(
                        PrimitiveStyleBuilder::new()
                            .fill_color(color)
                            .stroke_color(color.invert())
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
                        FontColor::Transparent(color.into()),
                        target,
                    );
                }
            }
        }
    }
}
