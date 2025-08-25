use kaolin::{
    Kaolin,
    commands::RenderCommand,
    fixed, sizing,
    style::{FlexStyle, TextStyle, border::Border},
};

mod common;
use common::*;

/// Tests rendering of a standalone text element without any containers.
/// Verifies basic text rendering with correct positioning and styling.
#[test]
fn render_text_element() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| k.text("Hello, Kaolin!", TextStyle::new()));

    assert_eq!(
        commands.collect::<Vec<_>>(),
        vec![RenderCommand::DrawText {
            text: "Hello, Kaolin!".to_string(),
            x: 0.0,
            y: 0.0,
            font_id: 0,
            font_size: 16.0,
            color: TestColor::Black,
        },]
    );
}

/// Tests rendering of an empty flex container with fixed dimensions.
/// Ensures that containers generate proper rectangle commands even when empty.
#[test]
fn render_empty_flex_container() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new().sizing(sizing!(fixed!(800.0), fixed!(600.0))),
            |k| k,
        )
    });

    assert_eq!(
        commands.collect::<Vec<_>>(),
        vec![RenderCommand::DrawRectangle {
            id: "".to_string(),
            color: TestColor::Transparent,
            x: 0.0,
            y: 0.0,
            width: 800.0,
            height: 600.0,
            corner_radius: 0.0,
            border: Border::default(),
        },]
    );
}

/// Tests rendering of text with custom font settings.
/// Verifies that font_id and font_size are properly applied to text elements.
#[test]
fn render_text_with_custom_font() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.text("Custom Font", TextStyle::new().font_id(5).font_size(24.0))
    });

    assert_eq!(
        commands.collect::<Vec<_>>(),
        vec![RenderCommand::DrawText {
            text: "Custom Font".to_string(),
            x: 0.0,
            y: 0.0,
            font_id: 5,
            font_size: 24.0,
            color: TestColor::Black,
        },]
    );
}

/// Tests rendering of a flex container with a text child.
/// Ensures that both container and content are rendered in the correct order.
#[test]
fn render_container_with_text() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new().sizing(sizing!(fixed!(200.0), fixed!(100.0))),
            |k| k.text("Inside", TextStyle::new()),
        )
    });

    assert_eq!(
        commands.collect::<Vec<_>>(),
        vec![
            RenderCommand::DrawRectangle {
                id: "".to_string(),
                color: TestColor::Transparent,
                x: 0.0,
                y: 0.0,
                width: 200.0,
                height: 100.0,
                corner_radius: 0.0,
                border: Border::default(),
            },
            RenderCommand::DrawText {
                text: "Inside".to_string(),
                x: 0.0,
                y: 0.0,
                font_id: 0,
                font_size: 16.0,
                color: TestColor::Black,
            },
        ]
    );
}

/// Tests rendering order with multiple text elements.
/// Verifies that elements are rendered in the order they are added to the layout.
#[test]
fn render_multiple_text_elements() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.text("First", TextStyle::new())
            .text("Second", TextStyle::new())
            .text("Third", TextStyle::new())
    });

    assert_eq!(
        commands.collect::<Vec<_>>(),
        vec![
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
                x: 50.0, // First text width (5 chars * 10px)
                y: 0.0,
                font_id: 0,
                font_size: 16.0,
                color: TestColor::Black,
            },
            RenderCommand::DrawText {
                text: "Third".to_string(),
                x: 110.0, // First + Second width (5 + 6 chars * 10px)
                y: 0.0,
                font_id: 0,
                font_size: 16.0,
                color: TestColor::Black,
            },
        ]
    );
}

/// Tests fit-to-content sizing behavior.
/// Verifies that containers automatically size themselves to their content when no explicit size is given.
#[test]
fn render_fit_to_content_container() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.with(FlexStyle::new(), |k| {
            k.text("Auto Size", TextStyle::new()) // 90px wide, 20px tall
        })
    });

    assert_eq!(
        commands.collect::<Vec<_>>(),
        vec![
            RenderCommand::DrawRectangle {
                id: "".to_string(),
                color: TestColor::Transparent,
                x: 0.0,
                y: 0.0,
                width: 90.0, // Fits text width
                height: 20.0, // Fits text height
                corner_radius: 0.0,
                border: Border::default(),
            },
            RenderCommand::DrawText {
                text: "Auto Size".to_string(),
                x: 0.0,
                y: 0.0,
                font_id: 0,
                font_size: 16.0,
                color: TestColor::Black,
            },
        ]
    );
}
