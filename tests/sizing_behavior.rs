use kaolin::{
    Kaolin,
    commands::RenderCommand,
    fixed, grow, sizing,
    style::{FlexStyle, border::Border},
};

mod common;
use common::*;

/// Tests sizing behavior of elements, including growable and fixed sizing.
/// Ensures that elements respect their size constraints and grow factors.
#[test]
fn growable_and_fixed_sizing() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.with(FlexStyle::new().sizing(sizing! { grow!() }), |k| k)
            .with(
                FlexStyle::new().sizing(sizing! {grow!(3.0), fixed!(200.0)}),
                |k| k,
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
                width: 200.0,
                height: 600.0,
                corner_radius: 0.0,
                border: Border::default(),
            },
            RenderCommand::DrawRectangle {
                id: "".to_string(),
                color: TestColor::Transparent,
                x: 200.0,
                y: 0.0,
                width: 600.0,
                height: 200.0,
                corner_radius: 0.0,
                border: Border::default(),
            },
        ],
    );
}

#[test]
fn minimum_and_maximum_sizing() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new().sizing(sizing! { fixed!(100.0), fixed!(50.0) }),
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
            width: 100.0,
            height: 50.0,
            corner_radius: 0.0,
            border: Border::default(),
        }],
    );
}
