use kaolin::{
    grow,
    renderers::KaolinRenderer,
    sizing,
    style::{
        FlexStyle, TextStyle,
        layout::{Alignment, Justification, Layout},
    },
};

#[cfg(feature = "raylib")]
fn main() {
    use kaolin::renderers::raylib::RaylibRenderer;

    let mut raylib_renderer = RaylibRenderer::new(800, 600);
    while !raylib_renderer.should_close() {
        raylib_renderer.draw(|k| {
            use raylib::color::Color;

            k.with(
                FlexStyle::new()
                    .background_color(Color::WHITE)
                    .layout(
                        Layout::new()
                            .alignment(Alignment::Center)
                            .justification(Justification::Center),
                    )
                    .sizing(sizing!(grow!())),
                |k| {
                    k.text(
                        "Hello, World!",
                        TextStyle::new().font_size(48.0).color(Color::BLACK),
                    )
                },
            )
        });
    }
}
