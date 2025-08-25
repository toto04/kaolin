use kaolin::{
    Kaolin, fixed, grow, sizing,
    style::{
        FlexStyle, TextStyle,
        layout::{Direction, Justification, Layout},
        padding::Padding,
    },
};

mod common;
use common::*;

/// Tests basic gap functionality between horizontal elements.
/// Verifies that the gap property correctly spaces out child elements.
#[test]
fn basic_horizontal_gaps() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(800.0), fixed!(600.0)))
                .layout(Layout::new().gap(20.0)),
            |k| {
                k.text("First", TextStyle::new()) // 50px wide
                    .text("Second", TextStyle::new()) // 60px wide
            },
        )
    });

    assert_size!(commands.next(), (800.0, 600.0));
    assert_position!(commands.next(), (0.0, 0.0)); // First text
    assert_position!(commands.next(), (70.0, 0.0)); // Second text (50 + 20 gap)
}

/// Tests gaps in vertical layout direction.
/// Ensures that gaps work correctly when elements are stacked vertically.
#[test]
fn vertical_gaps() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(800.0), fixed!(600.0)))
                .layout(Layout::new().direction(Direction::TopToBottom).gap(15.0)),
            |k| {
                k.text("Top", TextStyle::new()) // 20px tall
                    .text("Middle", TextStyle::new()) // 20px tall
                    .text("Bottom", TextStyle::new()) // 20px tall
            },
        )
    });

    assert_size!(commands.next(), (800.0, 600.0));
    assert_position!(commands.next(), (0.0, 0.0)); // Top text
    assert_position!(commands.next(), (0.0, 35.0)); // Middle text (20 + 15 gap)
    assert_position!(commands.next(), (0.0, 70.0)); // Bottom text (20 + 15 + 20 + 15)
}

/// Tests interaction between gaps and justification.
/// Verifies that SpaceBetween justification overrides gap when it results in larger spacing.
#[test]
fn gaps_with_space_between_justification() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(800.0), fixed!(600.0)))
                .layout(
                    Layout::new()
                        .gap(10.0) // Small gap
                        .justification(Justification::SpaceBetween),
                ),
            |k| {
                k.text("Left", TextStyle::new()) // 40px wide
                    .text("Right", TextStyle::new()) // 50px wide
            },
        )
    });

    assert_size!(commands.next(), (800.0, 600.0));
    assert_position!(commands.next(), (0.0, 0.0)); // Left text
    // SpaceBetween creates larger gap than 10px: (800 - 90) = 710px between elements
    assert_position!(commands.next(), (750.0, 0.0)); // Right text
}

/// Tests gaps with space around justification.
/// Ensures that SpaceAround correctly distributes space including the specified gap.
#[test]
fn gaps_with_space_around_justification() {
    let kaolin = Kaolin::new((600, 400), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(600.0), fixed!(400.0)))
                .layout(
                    Layout::new()
                        .gap(20.0)
                        .justification(Justification::SpaceAround),
                ),
            |k| {
                k.text("A", TextStyle::new()) // 10px wide
                    .text("B", TextStyle::new()) // 10px wide
            },
        )
    });

    assert_size!(commands.next(), (600.0, 400.0));
    // Total content width = 20px, available = 580px
    // SpaceAround gap = 580/3 = 193.33..px (max of this and 20px gap)
    assert_position!(commands.next(), (580.0 / 3.0, 0.0)); // A
    assert_position!(commands.next(), (580.0 / 3.0 * 2.0 + 10.0, 0.0)); // B (145 + 10 + 145 + 145)
}

/// Tests large gaps that might affect layout significantly.
/// Verifies that very large gaps are handled correctly without breaking layout.
#[test]
fn large_gaps() {
    let kaolin = Kaolin::new((500, 300), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(500.0), fixed!(300.0)))
                .layout(Layout::new().gap(200.0)), // Very large gap
            |k| {
                k.text("Left", TextStyle::new()) // 40px wide
                    .text("Right", TextStyle::new()) // 50px wide
            },
        )
    });

    assert_size!(commands.next(), (500.0, 300.0));
    assert_position!(commands.next(), (0.0, 0.0)); // Left text, vertically centered
    assert_position!(commands.next(), (240.0, 0.0)); // Right text (40 + 200 gap)
}

/// Tests gaps combined with padding.
/// Ensures that gaps and padding work together correctly without interfering.
#[test]
fn gaps_with_padding() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(800.0), fixed!(600.0)))
                .padding(Padding::all(30.0))
                .layout(Layout::new().gap(15.0)),
            |k| {
                k.text("First", TextStyle::new()) // 50px wide
                    .text("Second", TextStyle::new()) // 60px wide
            },
        )
    });

    assert_size!(commands.next(), (800.0, 600.0));
    // Positioning accounts for padding
    assert_position!(commands.next(), (30.0, 30.0)); // First text (padding offset)
    assert_position!(commands.next(), (95.0, 30.0)); // Second text (30 + 50 + 15 gap)
}

/// Tests gaps in mixed fixed and growable layouts.
/// Verifies that gaps are correctly maintained when elements have different sizing behaviors.
#[test]
fn gaps_with_mixed_sizing() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(800.0), fixed!(600.0)))
                .layout(Layout::new().gap(25.0)),
            |k| {
                k.with(FlexStyle::new().sizing(sizing!(fixed!(100.0))), |k| k)
                    .with(FlexStyle::new().sizing(sizing!(grow!(1.0))), |k| k)
                    .with(FlexStyle::new().sizing(sizing!(fixed!(150.0))), |k| k)
            },
        )
    });

    assert_size!(commands.next(), (800.0, 600.0)); // Container
    assert_position!(commands.next(), (0.0, 0.0)); // Fixed 100px element
    // Growable element: total - fixed elements - gaps = 800 - 250 - 50 = 500px
    assert_multiple!(
        commands.next(),
        assert_size((500.0, 600.0)),
        assert_position((125.0, 0.0)) // 100 + 25 gap
    );
    assert_multiple!(
        commands.next(),
        assert_size((150.0, 150.0)),
        assert_position((650.0, 0.0)) // 100 + 25 + 500 + 25
    );
}

/// Tests zero gaps (should behave like no gap specified).
/// Ensures that explicitly setting gap to 0 works correctly.
#[test]
fn zero_gaps() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(800.0), fixed!(600.0)))
                .layout(Layout::new().gap(0.0)),
            |k| {
                k.text("Adjacent", TextStyle::new()) // 80px wide
                    .text("Elements", TextStyle::new()) // 80px wide
            },
        )
    });

    assert_size!(commands.next(), (800.0, 600.0));
    assert_position!(commands.next(), (0.0, 0.0)); // First text
    assert_position!(commands.next(), (80.0, 0.0)); // Second text (no gap)
}
