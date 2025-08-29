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

    use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
    use embedded_graphics_simulator::{
        BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
    };
    use kaolin::renderers::embedded::EmbeddedRenderer;
    use u8g2_fonts::{FontRenderer, fonts};

    const FONTS: [FontRenderer; 2] = [
        FontRenderer::new::<fonts::u8g2_font_4x6_tf>(),
        FontRenderer::new::<fonts::u8g2_font_profont10_mr>(),
    ];

    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));
    let renderer = EmbeddedRenderer::new(&FONTS, display.bounding_box());
    renderer.onto(&mut display).draw(|k| {
        k.with(
            FlexStyle::new()
                .background_color(BinaryColor::Off)
                .layout(
                    Layout::new()
                        .alignment(Alignment::Center)
                        .justification(Justification::Center),
                )
                .sizing(sizing!(grow!())),
            |k| {
                k.text(
                    "Hello, World!",
                    TextStyle::new().font_size(48.0).color(BinaryColor::On),
                )
            },
        )
    });

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    Window::new("Hello World", &output_settings).show_static(&display);
}
