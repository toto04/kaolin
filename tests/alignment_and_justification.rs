use kaolin::{
    Kaolin, fixed, sizing,
    style::{
        FlexStyle, TextStyle,
        layout::{Alignment, Direction, Justification, Layout},
        padding::Padding,
    },
};

mod common;
use common::*;

/// Tests basic center alignment with space-between justification in horizontal layout.
/// This verifies that elements are vertically centered and horizontally spaced to the edges.
#[test]
fn horizontal_center_alignment_space_between() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
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

    assert_size!(commands.next(), (800.0, 600.0));
    assert_position!(commands.next(), (0.0, 290.0)); // Left text, vertically centered
    assert_position!(commands.next(), (750.0, 290.0)); // Right text, at the end
}

/// Tests start alignment with start justification in vertical layout.
/// Ensures elements align to the top-left corner with no extra spacing.
#[test]
fn vertical_start_alignment_start_justification() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(800.0), fixed!(600.0)))
                .layout(
                    Layout::new()
                        .direction(Direction::TopToBottom)
                        .alignment(Alignment::Start)
                        .justification(Justification::Start),
                ),
            |k| {
                k.text("Top Left", TextStyle::new())
                    .text("Below", TextStyle::new())
            },
        )
    });

    assert_size!(commands.next(), (800.0, 600.0));
    assert_position!(commands.next(), (0.0, 0.0));
    assert_position!(commands.next(), (0.0, 20.0));
}

/// Tests end alignment with end justification in vertical layout.
/// Verifies that elements are positioned at the bottom-right with proper text width calculation.
#[test]
fn vertical_end_alignment_end_justification() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(800.0), fixed!(600.0)))
                .layout(
                    Layout::new()
                        .direction(Direction::TopToBottom)
                        .alignment(Alignment::End)
                        .justification(Justification::End),
                ),
            |k| {
                k.text("Above", TextStyle::new()) // 50px wide
                    .text("Bottom Right", TextStyle::new()) // 120px wide
            },
        )
    });

    assert_size!(commands.next(), (800.0, 600.0));
    assert_position!(commands.next(), (750.0, 560.0)); // "Above" text, right-aligned
    assert_position!(commands.next(), (680.0, 580.0)); // "Bottom Right" text, right-aligned
}

/// Tests center alignment with center justification in horizontal layout.
/// Verifies that elements are perfectly centered both horizontally and vertically.
#[test]
fn horizontal_center_alignment_center_justification() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(800.0), fixed!(600.0)))
                .layout(
                    Layout::new()
                        .alignment(Alignment::Center)
                        .justification(Justification::Center),
                ),
            |k| {
                k.text("Center", TextStyle::new()) // 60px wide
                    .text("Middle", TextStyle::new()) // 60px wide
            },
        )
    });

    assert_size!(commands.next(), (800.0, 600.0));
    // Total width of texts = 120px, centered in 800px = (800-120)/2 = 340px start
    assert_position!(commands.next(), (340.0, 290.0));
    assert_position!(commands.next(), (400.0, 290.0));
}

/// Tests space-around justification with multiple elements.
/// Ensures equal spacing is distributed around all elements, including the edges.
#[test]
fn horizontal_space_around_justification() {
    let kaolin = Kaolin::new((900, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(900.0), fixed!(600.0)))
                .layout(
                    Layout::new()
                        .alignment(Alignment::Center)
                        .justification(Justification::SpaceAround),
                ),
            |k| {
                k.text("A", TextStyle::new()) // 10px wide
                    .text("B", TextStyle::new()) // 10px wide
                    .text("C", TextStyle::new()) // 10px wide
            },
        )
    });

    assert_size!(commands.next(), (900.0, 600.0));
    // Total text width = 30px, available space = 870px
    // SpaceAround divides by (n_gaps + 2) = 4, so gap = 217.5px
    assert_position!(commands.next(), (217.5, 290.0)); // A
    assert_position!(commands.next(), (445.0, 290.0)); // B (217.5 + 10 + 217.5)
    assert_position!(commands.next(), (672.5, 290.0)); // C
}

/// Tests different directions with complex alignment combinations.
/// Verifies that right-to-left and bottom-to-top directions work correctly.
#[test]
#[ignore = "reason: implement RightToLeft!"]
fn different_directions_complex_alignment() {
    let kaolin = Kaolin::new((400, 300), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(400.0), fixed!(300.0)))
                .layout(
                    Layout::new()
                        .direction(Direction::RightToLeft)
                        .alignment(Alignment::End)
                        .justification(Justification::Start),
                ),
            |k| {
                k.text("First", TextStyle::new()) // 50px wide
                    .text("Second", TextStyle::new()) // 60px wide
            },
        )
    });

    assert_size!(commands.next(), (400.0, 300.0));
    // RightToLeft starts from the right, End alignment = bottom
    assert_position!(commands.next(), (350.0, 280.0)); // First text
    assert_position!(commands.next(), (290.0, 280.0)); // Second text
}

/// Tests stretch alignment in a vertical layout.
/// Ensures that child elements expand to fill the cross-axis (width in vertical layout).
#[test]
#[ignore = "reason: implement stretch?"]
fn vertical_stretch_alignment() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(800.0), fixed!(600.0)))
                .layout(
                    Layout::new()
                        .direction(Direction::TopToBottom)
                        .alignment(Alignment::Stretch),
                ),
            |k| k.with(FlexStyle::new(), |k| k.text("Stretched", TextStyle::new())),
        )
    });

    assert_size!(commands.next(), (800.0, 600.0)); // Container
    assert_size!(commands.next(), (800.0, 20.0)); // Child stretched to full width
    assert_position!(commands.next(), (0.0, 0.0)); // Child position
}

/// Tests alignment and justification with padding.
/// Verifies that padding is correctly accounted for in positioning calculations.
#[test]
fn alignment_with_padding() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(800.0), fixed!(600.0)))
                .padding(Padding::all(50.0))
                .layout(
                    Layout::new()
                        .alignment(Alignment::Center)
                        .justification(Justification::Center),
                ),
            |k| {
                k.text("Padded", TextStyle::new()) // 60px wide
            },
        )
    });

    assert_size!(commands.next(), (800.0, 600.0));
    // Available space after padding = 700x500, text centered = (700-60)/2 + 50 = 370
    assert_position!(commands.next(), (370.0, 290.0));
}
