use kaolin::{
    Kaolin,
    commands::RenderCommand,
    fixed, sizing,
    style::{FlexStyle, TextStyle, border::Border},
};

mod common;
use common::*;

/// Tests rendering of nested flex containers.
/// Ensures that child containers are rendered correctly within their parent containers.
#[test]
fn nested_flex_containers() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new().sizing(sizing!(fixed!(800.0), fixed!(600.0))),
            |k| {
                k.with(
                    FlexStyle::new().sizing(sizing!(fixed!(400.0), fixed!(300.0))),
                    |k| k.text("Nested Text", TextStyle::new()),
                )
            },
        )
    });

    assert_render_commands(
        commands,
        vec![
            RenderCommand::DrawRectangle {
                id: "".to_string(),
                color: TestColor::Transparent,
                x: 0.0,
                y: 0.0,
                width: 800.0,
                height: 600.0,
                corner_radius: 0.0,
                border: Border::default(),
            },
            RenderCommand::DrawRectangle {
                id: "".to_string(),
                color: TestColor::Transparent,
                x: 0.0,
                y: 0.0,
                width: 400.0,
                height: 300.0,
                corner_radius: 0.0,
                border: Border::default(),
            },
            RenderCommand::DrawText {
                text: "Nested Text".to_string(),
                x: 0.0,
                y: 0.0,
                font_id: 0,
                font_size: 16.0,
                color: TestColor::Black,
            },
        ],
    );
}
