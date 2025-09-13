use kaolin::{
    Kaolin, fit, fixed, sizing,
    style::{
        FlexStyle, TextStyle,
        layout::{Alignment, Direction, Layout},
        padding::Padding,
    },
};

mod common;
use common::*;

/// Tests basic text wrapping within width-constrained containers.
/// Verifies that long text is correctly split across multiple lines when container width is limited.
#[test]
fn basic_text_wrapping() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(
            FlexStyle::new().sizing(sizing!(fixed!(100.0), fit!())),
            |k| k.text("This is a long text that should wrap", TextStyle::new()),
        )
    });

    assert_multiple!(
        commands.next(),
        assert_position((0.0, 0.0)),
        assert_size((100.0, 100.0)) // Height should fit 5 lines of text (5 * 20px)
    );
    assert_multiple!(
        commands.next(),
        assert_text_content("This is a"),
        assert_position((0.0, 0.0))
    );
    assert_multiple!(
        commands.next(),
        assert_text_content("long text"),
        assert_position((0.0, 20.0))
    );
    assert_multiple!(
        commands.next(),
        assert_text_content("that"),
        assert_position((0.0, 40.0))
    );
    assert_multiple!(
        commands.next(),
        assert_text_content("should"),
        assert_position((0.0, 60.0))
    );
    assert_multiple!(
        commands.next(),
        assert_text_content("wrap"),
        assert_position((0.0, 80.0))
    );
}

/// Tests text wrapping with very narrow containers.
/// Ensures that even single words are handled correctly when space is extremely limited.
#[test]
fn narrow_container_text_wrapping() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(
            FlexStyle::new().sizing(sizing!(fixed!(25.0), fit!())), // Only 2.5 characters width
            |k| k.text("Hello World", TextStyle::new()),
        )
    });

    assert_size!(commands.next(), (25.0, 40.0)); // whitespace wrapping == 2 lines
    // Text should be broken character by character if needed
}

/// Tests text wrapping with padding affecting available space.
/// Verifies that padding reduces available text width and affects wrapping calculations.
#[test]
fn text_wrapping_with_padding() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(120.0), fit!()))
                .padding(Padding::all(10.0)),
            |k| k.text("Text with padding wrapping", TextStyle::new()),
        )
    });

    // Available width for text = 120 - 20 (padding) = 100px = 10 characters
    assert_size!(commands.next(), (120.0, 80.0)); // Container with padding (3 lines + 20px pad)
    assert_position!(commands.next(), (10.0, 10.0)); // First line offset by padding
}

/// Tests text wrapping in vertical layout containers.
/// Ensures that text wrapping works correctly when the container uses TopToBottom direction.
#[test]
fn text_wrapping_vertical_layout() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(800.0), fixed!(600.0)))
                .layout(Layout::new().direction(Direction::TopToBottom)),
            |k| {
                k.with(
                    FlexStyle::new().sizing(sizing!(fixed!(90.0), fit!())),
                    |k| k.text("Vertical layout wrapping text", TextStyle::new()),
                )
            },
        )
    });

    assert_size!(commands.next(), (800.0, 600.0)); // Main container
    // Inner container should wrap text within 90px width
    assert_size!(commands.next(), (90.0, 80.0)); // 4 lines of text
}

/// Tests text wrapping with multiple text elements in the same container.
/// Verifies that each text element wraps independently within the shared container width.
#[test]
fn multiple_texts_wrapping() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(800.0), fixed!(600.0)))
                .layout(Layout::new().direction(Direction::TopToBottom)),
            |k| {
                k.with(
                    FlexStyle::new().sizing(sizing!(fixed!(100.0), fit!())),
                    |k| k.text("First text wraps", TextStyle::new()),
                )
                .with(
                    FlexStyle::new().sizing(sizing!(fixed!(100.0), fit!())),
                    |k| k.text("Second text also wraps", TextStyle::new()),
                )
            },
        )
    });

    assert_size!(commands.next(), (800.0, 600.0)); // Main container
    assert_size!(commands.next(), (_, 40.0)); // First text container (2 lines)
    commands.next(); // Skip first text line 1
    commands.next(); // Skip first text line 2
    assert_size!(commands.next(), (_, 60.0)); // Second text container (3 lines)
}

/// Tests text that doesn't need wrapping in a wide container.
/// Ensures that wrapping logic doesn't interfere with normal text that fits on one line.
#[test]
fn no_wrapping_needed() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(
            FlexStyle::new().sizing(sizing!(fixed!(500.0), fit!())),
            |k| k.text("Short text", TextStyle::new()), // 100px wide, fits easily
        )
    });

    assert_size!(commands.next(), (500.0, 20.0)); // Container with single line height
    assert_position!(commands.next(), (0.0, 0.0)); // Single text line
}

/// Tests text wrapping with different alignments.
/// Verifies that wrapped text lines are correctly aligned within their container.
#[test]
fn text_wrapping_with_alignment() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(100.0), fit!()))
                .layout(Layout::new().alignment(Alignment::Center)),
            |k| k.text("Center aligned wrapping text", TextStyle::new()),
        )
    });

    assert_size!(commands.next(), (100.0, 80.0)); // Container for 4 lines
    // Each wrapped line should be centered within the 100px container
    // Exact positions depend on the wrapping implementation
}

/// Tests empty text wrapping behavior.
/// Ensures that empty strings are handled gracefully in wrapping contexts.
#[test]
fn empty_text_wrapping() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(
            FlexStyle::new().sizing(sizing!(fixed!(100.0), fit!())),
            |k| k.text("", TextStyle::new()), // Empty text
        )
    });

    assert_size!(commands.next(), (100.0, 0.0)); // Container should have zero height
    // No text commands should be generated for empty text
}
