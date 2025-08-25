use kaolin::{
    Kaolin, fit, fixed, grow, sizing,
    style::{FlexStyle, TextStyle, layout::Layout},
};

mod common;
use common::*;

/// Tests error handling for negative fixed sizing values.
/// Ensures that the system properly rejects invalid negative dimensions.
#[test]
#[should_panic(expected = "Negative")]
fn negative_fixed_width() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    kaolin.draw(|k| {
        k.with(
            FlexStyle::new().sizing(sizing!(fixed!(-100.0), fixed!(50.0))),
            |k| k,
        )
    });
}

/// Tests error handling for negative fixed height values.
/// Ensures that negative height values are properly rejected.
#[test]
#[should_panic(expected = "Negative")]
fn negative_fixed_height() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    kaolin.draw(|k| {
        k.with(
            FlexStyle::new().sizing(sizing!(fixed!(100.0), fixed!(-50.0))),
            |k| k,
        )
    });
}

/// Tests error handling for negative growth factors.
/// Verifies that negative growth factors are rejected by the system.
#[test]
#[should_panic(expected = "Negative")]
fn negative_growth_factor() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    kaolin.draw(|k| k.with(FlexStyle::new().sizing(sizing!(grow!(-1.0))), |k| k));
}

/// Tests behavior with extremely large fixed sizes.
/// Verifies that the system handles very large (but valid) dimensions without panicking.
#[test]
fn very_large_fixed_sizes() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new().sizing(sizing!(fixed!(1_000_000.0), fixed!(1_000_000.0))),
            |k| k,
        )
    });

    // Should not panic, but create a very large container
    assert_size!(commands.next(), (1_000_000.0, 1_000_000.0));
}

/// Tests behavior with very small but positive fixed sizes.
/// Ensures that very small positive values are handled correctly.
#[test]
fn very_small_positive_sizes() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new().sizing(sizing!(fixed!(0.001), fixed!(0.001))),
            |k| k,
        )
    });

    // Should not panic and should respect the tiny size
    assert_size!(commands.next(), (0.001, 0.001));
}

/// Tests error handling with invalid constraint combinations in fit sizing.
/// Verifies that impossible fit constraints are handled gracefully.
#[test]
#[should_panic(expected = "Negative")]
fn invalid_fit_constraints() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    kaolin.draw(|k| {
        k.with(
            FlexStyle::new().sizing(sizing!(fit!(-10.0, 100.0))), // negative min
            |k| k,
        )
    });
}

/// Tests handling of conflicting sizing requirements.
/// Verifies that impossible sizing combinations are detected and handled.
#[test]
fn conflicting_sizing_requirements() {
    let kaolin = Kaolin::new((100, 100), measure_text); // Small viewport
    let mut commands = kaolin.draw(|k| {
        k.with(FlexStyle::new().sizing(sizing!(fixed!(200.0))), |k| k) // Larger than viewport
            .with(FlexStyle::new().sizing(sizing!(fixed!(200.0))), |k| k) // Another large element
    });

    // Should not panic but may produce overlapping or constrained elements
    // The exact behavior depends on the shrinking implementation
    assert_size!(commands.next(), (200.0, 200.0)); // First element
    assert_size!(commands.next(), (200.0, 200.0)); // Second element
}

/// Tests error handling with malformed text input.
/// Ensures that various text inputs don't cause panics in the rendering system.
#[test]
fn malformed_text_input() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.with(FlexStyle::new()
        .sizing(sizing!(grow!()))
        .layout(
          Layout::new()
          .direction(kaolin::style::layout::Direction::TopToBottom)
        ), |k| {
            k.text("", TextStyle::new()) // Empty string
                .text("Normal text", TextStyle::new())
                .text("Text\nwith\nnewlines", TextStyle::new()) // Multi-line
                .text("Very long text that exceeds normal expectations for a single text element and might cause issues", TextStyle::new())
        })
    });

    // Should handle all text inputs gracefully without panicking
    let mut commands = commands.skip(1);
    assert_text_content!(commands.next(), "Normal text");
    assert_text_content!(commands.next(), "Text");
    assert_text_content!(commands.next(), "with");
    assert_text_content!(commands.next(), "newlines");
    assert_text_content!(
        commands.next(),
        "Very long text that exceeds normal expectations for a single text element and"
    );
    assert_text_content!(commands.next(), "might cause issues");
}

/// Tests recovery from invalid intermediate states.
/// Verifies that the system can handle edge cases in layout calculations.
#[test]
fn layout_calculation_edge_cases() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(800.0), fixed!(600.0)))
                .layout(Layout::new().gap(f64::MAX)), // Extremely large gap
            |k| {
                k.text("First", TextStyle::new())
                    .text("Second", TextStyle::new())
            },
        )
    });

    // Should not panic even with extreme gap values
    // Behavior may clamp or handle the large gap gracefully
    assert_size!(commands.next(), (800.0, 600.0)); // Container
}
