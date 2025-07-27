use kaolin::renderers::KaolinRenderer;
use kaolin::renderers::raylib::RaylibRenderer;
use kaolin::style::layout::{Alignment, Justification};
use kaolin::style::padding::Padding;
use kaolin::style::sizing::BoxSizing;
use kaolin::style::{FlexStyle, colors::Colors};
use kaolin::{fixed, flex_style, grow, layout, text_config};

fn main() {
    let mut renderer = RaylibRenderer::new(800, 600);
    while !renderer.should_close() {
        renderer.draw(|k| {
            k.with(
                flex_style! {
                    sizing: BoxSizing {
                        width: grow!(1.0),
                        height: grow!(1.0),
                    },
                    layout: layout! {
                        justification: Justification::Center,
                        alignment: Alignment::Center,
                        gap: 10.0,
                    },
                    padding: Padding::all(10.0),
                },
                |k| {
                    k.with(
                        flex_style! {
                          sizing: BoxSizing {
                            width: fixed!(200.0),
                            height: grow!(1.0),
                          },
                          background_color: Colors::Blue.into(),
                        },
                        |k| k,
                    )
                    .with(
                        flex_style! {
                          sizing: BoxSizing {
                            width: grow!(1.0),
                            height: grow!(1.0),
                          },
                          background_color: Colors::Green.into(),
                          layout: layout! {
                              justification: Justification::Center,
                              alignment: Alignment::Center,
                          },
                        },
                        |k| {
                            k.text(
                                "Hello, Raylib!",
                                text_config! {
                                    font_size: 20.0,
                                    color: Colors::White.into()
                                },
                            )
                        },
                    )
                },
            )
        });
    }
}
