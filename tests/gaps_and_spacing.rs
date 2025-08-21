use kaolin::{
    Kaolin,
    commands::RenderCommand,
    fixed, sizing,
    style::{FlexStyle, TextStyle, border::Border, layout::Layout},
};

mod common;
use common::*;

/// Tests gaps and spacing between elements in a flex container.
/// Verifies that the gap property is applied correctly to space out child elements.
#[test]
fn gaps_between_elements() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(800.0), fixed!(600.0)))
                .layout(Layout::new().gap(20.0)),
            |k| {
                k.text("First", TextStyle::new())
                    .text("Second", TextStyle::new())
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
            RenderCommand::DrawText {
                text: "First".to_string(),
                x: 0.0,
                y: 0.0,
                font_id: 0,
                font_size: 16.0,
                color: TestColor::Black,
            },
            RenderCommand::DrawText {
                text: "Second".to_string(),
                x: 70.0,
                y: 0.0,
                font_id: 0,
                font_size: 16.0,
                color: TestColor::Black,
            },
        ],
    );
}
