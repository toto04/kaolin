use kaolin::{
    Kaolin, fit, fixed, grow, sizing,
    style::{
        FlexStyle, TextStyle,
        layout::{Alignment, Direction, Justification, Layout},
        padding::Padding,
    },
};

mod common;
use common::*;

/// Tests basic nested container functionality.
/// Ensures that child containers are rendered correctly within their parent containers.
#[test]
fn basic_nested_containers() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new().sizing(sizing!(fixed!(800.0), fixed!(600.0))),
            |k| {
                k.with(
                    FlexStyle::new().sizing(sizing!(fixed!(400.0), fixed!(300.0))),
                    |k| k.text("Nested Text", TextStyle::new()),
                )
            },
        )
    });

    assert_multiple!(
        commands.next(),
        assert_size((800.0, 600.0)),
        assert_position((0.0, 0.0))
    );
    assert_multiple!(
        commands.next(),
        assert_size((400.0, 300.0)),
        assert_position((0.0, 0.0))
    );
    assert_multiple!(
        commands.next(),
        assert_text_content("Nested Text"),
        assert_position((0.0, 0.0))
    );
}

/// Tests multiple levels of nesting with different layout directions.
/// Verifies that complex nested structures maintain correct positioning and sizing.
#[test]
fn multi_level_nesting_different_directions() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(800.0), fixed!(600.0)))
                .layout(Layout::new().direction(Direction::LeftToRight)),
            |k| {
                k.with(
                    FlexStyle::new()
                        .sizing(sizing!(fixed!(400.0), fixed!(600.0)))
                        .layout(Layout::new().direction(Direction::TopToBottom)),
                    |k| {
                        k.with(
                            FlexStyle::new().sizing(sizing!(fixed!(400.0), fixed!(200.0))),
                            |k| k.text("Top", TextStyle::new()),
                        )
                        .with(
                            FlexStyle::new().sizing(sizing!(fixed!(400.0), fixed!(200.0))),
                            |k| k.text("Bottom", TextStyle::new()),
                        )
                    },
                )
                .with(
                    FlexStyle::new().sizing(sizing!(fixed!(400.0), fixed!(600.0))),
                    |k| k.text("Right Side", TextStyle::new()),
                )
            },
        )
    });

    assert_size!(commands.next(), (800.0, 600.0)); // Main container
    assert_size!(commands.next(), (400.0, 600.0)); // Left vertical container
    assert_multiple!(
        commands.next(),
        assert_size((400.0, 200.0)), // Top child
        assert_position((0.0, 0.0))
    );
    assert_position!(commands.next(), (0.0, 0.0)); // "Top" text
    assert_multiple!(
        commands.next(),
        assert_size((400.0, 200.0)), // Bottom child
        assert_position((0.0, 200.0))
    );
    assert_position!(commands.next(), (0.0, 200.0)); // "Bottom" text
    assert_multiple!(
        commands.next(),
        assert_size((400.0, 600.0)), // Right container
        assert_position((400.0, 0.0))
    );
    assert_position!(commands.next(), (400.0, 0.0)); // "Right Side" text
}

/// Tests nested containers with growth behavior.
/// Ensures that nested growing elements correctly distribute available space.
#[test]
fn nested_containers_with_growth() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(FlexStyle::new().sizing(sizing!(grow!(1.0))), |k| {
            k.with(FlexStyle::new().sizing(sizing!(grow!(1.0), fit!())), |k| {
                k.text("Inner Growth", TextStyle::new())
            })
        })
        .with(FlexStyle::new().sizing(sizing!(grow!(1.0))), |k| {
            k.text("Sibling", TextStyle::new())
        })
    });

    // Each top-level container gets 400px (800/2)
    assert_size!(commands.next(), (400.0, 600.0)); // First container
    assert_size!(commands.next(), (400.0, 20.0)); // Inner container (fits to text height)
    assert_position!(commands.next(), (0.0, 0.0)); // "Inner Growth" text
    assert_multiple!(
        commands.next(),
        assert_size((400.0, 600.0)), // Second container
        assert_position((400.0, 0.0))
    );
    assert_position!(commands.next(), (400.0, 0.0)); // "Sibling" text
}

/// Tests nested containers with different alignment settings.
/// Verifies that alignment properties are correctly applied in nested contexts.
#[test]
fn nested_containers_different_alignments() {
    let kaolin = Kaolin::new((600, 400), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(600.0), fixed!(400.0)))
                .layout(
                    Layout::new()
                        .alignment(Alignment::Center)
                        .justification(Justification::Center),
                ),
            |k| {
                k.with(
                    FlexStyle::new()
                        .sizing(sizing!(fixed!(200.0), fixed!(100.0)))
                        .layout(
                            Layout::new()
                                .alignment(Alignment::End)
                                .justification(Justification::End),
                        ),
                    |k| k.text("Nested", TextStyle::new()), // 60px wide
                )
            },
        )
    });

    assert_size!(commands.next(), (600.0, 400.0)); // Outer container
    // Centered in outer: (600-200)/2 = 200, (400-100)/2 = 150
    assert_multiple!(
        commands.next(),
        assert_size((200.0, 100.0)),
        assert_position((200.0, 150.0))
    );
    // Text positioned at end within inner container: 200-60=140, 100-20=80, plus inner position
    assert_position!(commands.next(), (340.0, 230.0));
}

/// Tests nested containers with padding affecting child positioning.
/// Ensures that padding cascades correctly through nested structures.
#[test]
fn nested_containers_with_padding() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(800.0), fixed!(600.0)))
                .padding(Padding::all(50.0)),
            |k| {
                k.with(
                    FlexStyle::new()
                        .sizing(sizing!(fixed!(300.0), fixed!(200.0)))
                        .padding(Padding::all(25.0)),
                    |k| k.text("Padded Text", TextStyle::new()),
                )
            },
        )
    });

    assert_size!(commands.next(), (800.0, 600.0)); // Outer container
    // Inner positioned within outer padding
    assert_multiple!(
        commands.next(),
        assert_size((300.0, 200.0)),
        assert_position((50.0, 50.0))
    );
    // Text positioned within both paddings: 50 + 25 = 75
    assert_position!(commands.next(), (75.0, 75.0));
}

/// Tests deeply nested containers (3+ levels).
/// Verifies that very deep nesting doesn't break layout calculations.
#[test]
fn deeply_nested_containers() {
    let kaolin = Kaolin::new((400, 300), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new().sizing(sizing!(fixed!(400.0), fixed!(300.0))),
            |k| {
                k.with(
                    FlexStyle::new().sizing(sizing!(fixed!(300.0), fixed!(200.0))),
                    |k| {
                        k.with(
                            FlexStyle::new().sizing(sizing!(fixed!(200.0), fixed!(100.0))),
                            |k| {
                                k.with(
                                    FlexStyle::new().sizing(sizing!(fixed!(100.0), fixed!(50.0))),
                                    |k| k.text("Deep", TextStyle::new()),
                                )
                            },
                        )
                    },
                )
            },
        )
    });

    assert_size!(commands.next(), (400.0, 300.0)); // Level 1
    assert_size!(commands.next(), (300.0, 200.0)); // Level 2
    assert_size!(commands.next(), (200.0, 100.0)); // Level 3
    assert_size!(commands.next(), (100.0, 50.0)); // Level 4
    assert_position!(commands.next(), (0.0, 0.0)); // Text at deepest level
}

/// Tests nested containers with mixed fit and fixed sizing.
/// Ensures that fit sizing works correctly within nested contexts.
#[test]
fn nested_fit_and_fixed_sizing() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new().sizing(sizing!(fixed!(800.0), fixed!(600.0))),
            |k| {
                k.with(
                    FlexStyle::new(), // Fit sizing
                    |k| {
                        k.text("Auto Size Content", TextStyle::new()) // 170px wide, 20px tall
                    },
                )
            },
        )
    });

    assert_size!(commands.next(), (800.0, 600.0)); // Outer fixed container
    assert_size!(commands.next(), (170.0, 20.0)); // Inner container fits to content
    assert_position!(commands.next(), (0.0, 0.0)); // Text
}
