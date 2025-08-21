use kaolin::{
    Kaolin,
    commands::RenderCommand,
    fixed, sizing,
    style::{
        FlexStyle, TextStyle,
        border::Border,
        layout::{Alignment, Justification, Layout},
    },
};

mod common;
use common::*;

/// Tests alignment and justification of elements within a flex container.
/// Ensures that elements are correctly spaced and aligned according to the specified layout properties.
#[test]
#[ignore = "i haven't done space between yet :("]
fn alignment_and_justification() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(800.0), fixed!(600.0)))
                .layout(
                    Layout::new()
                        .alignment(Alignment::Center)
                        .justification(Justification::SpaceBetween),
                ),
            |k| {
                k.text("Left", TextStyle::new())
                    .text("Right", TextStyle::new())
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
                text: "Left".to_string(),
                x: 0.0,
                y: 290.0,
                font_id: 0,
                font_size: 16.0,
                color: TestColor::Black,
            },
            RenderCommand::DrawText {
                text: "Right".to_string(),
                x: 760.0,
                y: 290.0,
                font_id: 0,
                font_size: 16.0,
                color: TestColor::Black,
            },
        ],
    );
}
