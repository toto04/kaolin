use kaolin::{
    Kaolin,
    style::{FlexStyle, TextStyle},
};

mod common;
use common::*;

/// Tests basic color inheritance from parent to child text elements.
/// Verifies that child elements inherit the color property from their parent when no explicit color is set.
#[test]
fn basic_color_inheritance() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(FlexStyle::new().color(TestColor::Red), |k| {
            k.text("Hello, Kaolin!", TextStyle::new())
        })
    });

    assert_color!(commands.nth(1), TestColor::Red);
}

/// Tests color inheritance through multiple levels of nesting.
/// Ensures that color inheritance works correctly through deeply nested container structures.
#[test]
fn multi_level_color_inheritance() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(FlexStyle::new().color(TestColor::Red), |k| {
            k.with(FlexStyle::new(), |k| {
                // No color set, should inherit
                k.with(FlexStyle::new(), |k| {
                    // Another level of nesting
                    k.text("Deeply Nested", TextStyle::new())
                })
            })
        })
    });

    // Skip containers and check text color
    commands.next(); // Outer container
    commands.next(); // Middle container  
    commands.next(); // Inner container
    assert_color!(commands.next(), TestColor::Red); // Text should inherit red
}

/// Tests color inheritance override at intermediate levels.
/// Verifies that a child container can override inherited color and pass its own color down.
#[test]
fn color_inheritance_override() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(FlexStyle::new().color(TestColor::Red), |k| {
            k.text("Red Text", TextStyle::new())
                .with(FlexStyle::new().color(TestColor::Black), |k| {
                    k.text("Black Text", TextStyle::new())
                })
        })
    });

    commands.next(); // Container
    assert_color!(commands.next(), TestColor::Red); // First text should be red
    commands.next(); // Inner container
    assert_color!(commands.next(), TestColor::Black); // Second text should be black
}

/// Tests explicit text color overriding inherited color.
/// Ensures that when a text element has an explicit color, it's not overridden by inheritance.
#[test]
fn explicit_text_color_override() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(FlexStyle::new().color(TestColor::Red), |k| {
            k.text("Inherited Red", TextStyle::new())
                .text("Explicit Black", TextStyle::new().color(TestColor::Black))
        })
    });

    commands.next(); // Container
    assert_color!(commands.next(), TestColor::Red); // Should inherit red
    assert_color!(commands.next(), TestColor::Black); // Should use explicit black
}

/// Tests color inheritance with multiple siblings.
/// Verifies that all siblings inherit the same color from their parent.
#[test]
fn color_inheritance_multiple_siblings() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(FlexStyle::new().color(TestColor::Red), |k| {
            k.text("First", TextStyle::new())
                .text("Second", TextStyle::new())
                .text("Third", TextStyle::new())
        })
    });

    commands.next(); // Container
    assert_color!(commands.next(), TestColor::Red); // First text
    assert_color!(commands.next(), TestColor::Red); // Second text
    assert_color!(commands.next(), TestColor::Red); // Third text
}

/// Tests default color behavior when no color is specified anywhere.
/// Ensures that default foreground color is used when no color inheritance is available.
#[test]
fn default_color_behavior() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(FlexStyle::new(), |k| {
            // No color specified
            k.text("Default Color", TextStyle::new())
        })
    });

    commands.next(); // Container
    assert_color!(commands.next(), TestColor::Black); // Should use default black
}

/// Tests color inheritance in complex nested structures with mixed color settings.
/// Verifies inheritance behavior in realistic complex layouts.
#[test]
fn complex_color_inheritance_scenario() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw::<()>(|k| {
        k.with(FlexStyle::new().color(TestColor::Red), |k| {
            k.text("Red Header", TextStyle::new())
                .with(FlexStyle::new(), |k| {
                    // Inherits red
                    k.text("Red Body", TextStyle::new())
                        .with(FlexStyle::new().color(TestColor::Black), |k| {
                            k.text("Black Footer", TextStyle::new())
                        })
                })
        })
    });

    commands.next(); // Outer container
    assert_color!(commands.next(), TestColor::Red); // Header
    commands.next(); // Middle container
    assert_color!(commands.next(), TestColor::Red); // Body (inherited)
    commands.next(); // Inner container
    assert_color!(commands.next(), TestColor::Black); // Footer (overridden)
}
