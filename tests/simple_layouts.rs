use kaolin::{
    Kaolin,
    commands::RenderCommand,
    fixed, grow, sizing,
    style::{
        FlexStyle, TextStyle,
        border::Border,
        layout::{Alignment, Direction, Justification, Layout},
    },
};

mod common;
use common::*;

/// Tests simple layout configurations, including alignment, justification, and direction.
/// Verifies that basic layout properties are applied correctly to elements.
#[test]
fn simple_layout() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(800.0), fixed!(600.0)))
                .layout(
                    Layout::new()
                        .direction(Direction::LeftToRight)
                        .alignment(Alignment::Center)
                        .justification(Justification::Center)
                        .gap(10.0),
                ),
            |k| k.text("Hello, Kaolin!", TextStyle::new()),
        )
    });

    assert_eq!(
        commands.collect::<Vec<_>>(),
        vec![
            RenderCommand::DrawRectangle {
                id: "".to_string(), // ID is not used in this context
                color: TestColor::Transparent,
                x: 0.0,
                y: 0.0,
                width: 800.0,
                height: 600.0,
                corner_radius: 0.0,
                border: Border::default()
            },
            RenderCommand::DrawText {
                text: "Hello, Kaolin!".to_string(),
                x: (800.0 - 140.0) / 2.0,
                y: (600.0 - 20.0) / 2.0,
                font_id: 0,
                font_size: 16.0,
                color: TestColor::Black
            },
        ]
    );
}

#[test]
fn double_growth() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.with(FlexStyle::new().sizing(sizing! { grow!() }), |k| k)
            .with(
                FlexStyle::new().sizing(sizing! {grow!(3.0), fixed!(200.0)}),
                |k| k,
            )
    });

    assert_eq!(
        commands.collect::<Vec<_>>(),
        vec![
            RenderCommand::DrawRectangle {
                id: "".to_string(), // ID is not used in this context
                color: TestColor::Transparent,
                x: 0.0,
                y: 0.0,
                width: 200.0,
                height: 600.0,
                corner_radius: 0.0,
                border: Border::default(),
            },
            RenderCommand::DrawRectangle {
                id: "".to_string(), // ID is not used in this context
                color: TestColor::Transparent,
                x: 200.0,
                y: 0.0,
                width: 600.0,
                height: 200.0,
                corner_radius: 0.0,
                border: Border::default(),
            },
        ]
    );
}

#[test]
fn fit_sizing() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.with(FlexStyle::new(), |k| {
            k.text("Hello, Kaolin!", TextStyle::new())
        })
    });

    assert_eq!(
        commands.collect::<Vec<_>>(),
        vec![
            RenderCommand::DrawRectangle {
                id: "".to_string(), // ID is not used in this context
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
                color: TestColor::Black,
            }
        ]
    );
}

#[test]
fn inherited_color() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.with(FlexStyle::new().color(TestColor::Red), |k| {
            k.text("Hello, Kaolin!", TextStyle::new())
        })
    });

    assert_eq!(
        commands.collect::<Vec<_>>(),
        vec![
            RenderCommand::DrawRectangle {
                id: "".to_string(), // ID is not used in this context
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
        ]
    );
}
