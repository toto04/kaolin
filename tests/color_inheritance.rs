use kaolin::{
    Kaolin,
    commands::RenderCommand,
    style::{FlexStyle, TextStyle, border::Border},
};

mod common;
use common::*;

/// Tests color inheritance within nested elements.
/// Verifies that child elements inherit the color property from their parent.
#[test]
fn inherited_color() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.with(FlexStyle::new().color(TestColor::Red), |k| {
            k.text("Hello, Kaolin!", TextStyle::new())
        })
    });

    assert_render_commands(
        commands,
        vec![
            RenderCommand::DrawRectangle {
                id: "".to_string(),
                color: TestColor::Transparent,
                x: 0.0,
                y: 0.0,
                width: 140.0,
                height: 20.0,
                corner_radius: 0.0,
                border: Border::default(),
            },
            RenderCommand::DrawText {
                text: "Hello, Kaolin!".to_string(),
                x: 0.0,
                y: 0.0,
                font_id: 0,
                font_size: 16.0,
                color: TestColor::Red, // Inherited color
            },
        ],
    );
}
