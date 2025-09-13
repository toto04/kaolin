use kaolin::{
    grow,
    renderers::KaolinRenderer,
    sizing,
    style::{
        FlexStyle, TextStyle,
        layout::{Alignment, Justification, Layout},
    },
};

#[cfg(feature = "embedded")]
fn main() {
    extern crate embedded_graphics_simulator;

    use embedded_graphics::{
        image::{Image, ImageRawBE},
        pixelcolor::BinaryColor,
        prelude::*,
    };
    use embedded_graphics_simulator::{
        BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
    };
    use kaolin::renderers::embedded::EmbeddedRenderer;
    use kaolin::style::layout::Direction;
    use u8g2_fonts::{FontRenderer, fonts};

    const FONTS: [FontRenderer; 2] = [
        FontRenderer::new::<fonts::u8g2_font_4x6_tf>(),
        FontRenderer::new::<fonts::u8g2_font_profont10_mr>(),
    ];

    let smily_face: [u8; 8] = [
        0b00111100, 0b01000010, 0b10100101, 0b10000001, 0b10100101, 0b10011001, 0b01000010,
        0b00111100,
    ];

    let raw = ImageRawBE::new(&smily_face, 8);
    let image = Image::new(&raw, (0, 0).into());

    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));
    let renderer = EmbeddedRenderer::new(&FONTS, display.bounding_box());
    renderer.onto(&mut display).draw(|k| {
        k.with(
            FlexStyle::new()
                .background_color(BinaryColor::Off)
                .layout(
                    Layout::new()
                        .direction(Direction::TopToBottom)
                        .alignment(Alignment::Center)
                        .gap(2.0)
                        .justification(Justification::Center),
                )
                .sizing(sizing!(grow!())),
            |k| {
                k.with(FlexStyle::new().layout(Layout::new().gap(4.0)), |k| {
                    k.text(
                        "Hello, World!",
                        TextStyle::new().font_id(1).color(BinaryColor::On),
                    )
                    .with_element(&image)
                })
                .text("Kaolin Renderer", TextStyle::new().color(BinaryColor::On))
            },
        )
    });

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    Window::new("Hello World", &output_settings).show_static(&display);
}
