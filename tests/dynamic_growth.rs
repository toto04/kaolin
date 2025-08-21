use kaolin::{
    Kaolin,
    commands::RenderCommand,
    grow, sizing,
    style::{FlexStyle, border::Border},
};

mod common;
use common::*;

/// Tests dynamic growth behavior of flex containers.
/// Ensures that elements grow proportionally based on their growth factors.
#[test]
fn dynamic_growth_behavior() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.with(FlexStyle::new().sizing(sizing! { grow!(1.0) }), |k| k)
            .with(FlexStyle::new().sizing(sizing! { grow!(2.0) }), |k| k)
    });

    assert_render_commands(
        commands,
        vec![
            RenderCommand::DrawRectangle {
                id: "".to_string(),
                color: TestColor::Transparent,
                x: 0.0,
                y: 0.0,
                width: 800.0 / 3.0,
                height: 600.0,
                corner_radius: 0.0,
                border: Border::default(),
            },
            RenderCommand::DrawRectangle {
                id: "".to_string(),
                color: TestColor::Transparent,
                x: 800.0 / 3.0,
                y: 0.0,
                width: 800.0 / 3.0 * 2.0,
                height: 600.0,
                corner_radius: 0.0,
                border: Border::default(),
            },
        ],
    );
}
