use kaolin::{
    Kaolin,
    commands::RenderCommand,
    fit, fixed, sizing,
    style::{FlexStyle, TextStyle, border::Border},
};

mod common;
use common::*;

/// Tests text wrapping behavior within constrained containers.
/// Verifies that long text is wrapped correctly to fit within the specified dimensions.
#[test]
fn text_wrapping_behavior() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new().sizing(sizing!(fixed!(100.0), fit!())),
            |k| k.text("This is a long text that should wrap", TextStyle::new()),
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
                width: 100.0,
                height: 100.0,
                corner_radius: 0.0,
                border: Border::default(),
            },
            RenderCommand::DrawText {
                text: "This is a".to_string(),
                x: 0.0,
                y: 0.0,
                font_id: 0,
                font_size: 16.0,
                color: TestColor::Black,
            },
            RenderCommand::DrawText {
                text: "long text".to_string(),
                x: 0.0,
                y: 20.0,
                font_id: 0,
                font_size: 16.0,
                color: TestColor::Black,
            },
            RenderCommand::DrawText {
                text: "that".to_string(),
                x: 0.0,
                y: 40.0,
                font_id: 0,
                font_size: 16.0,
                color: TestColor::Black,
            },
            RenderCommand::DrawText {
                text: "should".to_string(),
                x: 0.0,
                y: 60.0,
                font_id: 0,
                font_size: 16.0,
                color: TestColor::Black,
            },
            RenderCommand::DrawText {
                text: "wrap".to_string(),
                x: 0.0,
                y: 80.0,
                font_id: 0,
                font_size: 16.0,
                color: TestColor::Black,
            },
        ],
    )
}
