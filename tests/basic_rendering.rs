use kaolin::{
    Kaolin,
    commands::RenderCommand,
    fixed, sizing,
    style::{FlexStyle, TextStyle, border::Border},
};

mod common;
use common::*;

/// Tests basic rendering capabilities, including rendering text and flex containers.
/// Verifies that the rendering commands are generated correctly for simple elements.
#[test]
fn render_text_element() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| k.text("Hello, Kaolin!", TextStyle::new()));

    assert_render_commands(
        commands,
        vec![RenderCommand::DrawText {
            text: "Hello, Kaolin!".to_string(),
            x: 0.0,
            y: 0.0,
            font_id: 0,
            font_size: 16.0,
            color: TestColor::Black,
        }],
    );
}

#[test]
fn render_flex_container() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new().sizing(sizing!(fixed!(800.0), fixed!(600.0))),
            |k| k,
        )
    });

    assert_render_commands(
        commands,
        vec![RenderCommand::DrawRectangle {
            id: "".to_string(),
            color: TestColor::Transparent,
            x: 0.0,
            y: 0.0,
            width: 800.0,
            height: 600.0,
            corner_radius: 0.0,
            border: Border::default(),
        }],
    );
}
