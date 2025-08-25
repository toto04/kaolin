use kaolin::{
    Kaolin, fixed, sizing,
    style::{FlexStyle, TextStyle, border::Border, padding::Padding},
};

mod common;
use common::*;

/// Tests basic border rendering on flex containers.
/// Verifies that border width and color are correctly applied to container rendering commands.
#[test]
fn basic_border_rendering() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(200.0), fixed!(100.0)))
                .border(Border::new().width(5.0).color(TestColor::Red)),
            |k| k,
        )
    });

    let cmd = commands.next().unwrap();
    match cmd {
        kaolin::commands::RenderCommand::DrawRectangle { border, .. } => {
            assert_eq!(border.width, 5.0);
            assert_eq!(border.color, TestColor::Red);
        }
        _ => panic!("Expected DrawRectangle command"),
    }
}

/// Tests corner radius rendering on flex containers.
/// Ensures that corner radius values are correctly passed to rendering commands.
#[test]
fn corner_radius_rendering() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(200.0), fixed!(100.0)))
                .corner_radius(15.0),
            |k| k,
        )
    });

    let cmd = commands.next().unwrap();
    match cmd {
        kaolin::commands::RenderCommand::DrawRectangle { corner_radius, .. } => {
            assert_eq!(corner_radius, 15.0);
        }
        _ => panic!("Expected DrawRectangle command"),
    }
}

/// Tests background color application on flex containers.
/// Verifies that background colors are correctly applied to container rendering.
#[test]
fn background_color_rendering() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(200.0), fixed!(100.0)))
                .background_color(TestColor::Red),
            |k| k,
        )
    });

    let cmd = commands.next().unwrap();
    match cmd {
        kaolin::commands::RenderCommand::DrawRectangle { color, .. } => {
            assert_eq!(color, TestColor::Red);
        }
        _ => panic!("Expected DrawRectangle command"),
    }
}

/// Tests complex styling with multiple properties combined.
/// Ensures that border, corner radius, and background color work together correctly.
#[test]
fn combined_styling_properties() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(300.0), fixed!(200.0)))
                .background_color(TestColor::Red)
                .border(Border::new().width(3.0))
                .corner_radius(10.0),
            |k| k.text("Styled", TextStyle::new()),
        )
    });

    let cmd = commands.next().unwrap();
    match cmd {
        kaolin::commands::RenderCommand::DrawRectangle {
            color,
            corner_radius,
            border,
            width,
            height,
            ..
        } => {
            assert_eq!(color, TestColor::Red);
            assert_eq!(corner_radius, 10.0);
            assert_eq!(width, 300.0);
            assert_eq!(height, 200.0);
            assert_eq!(border.width, 3.0);
            assert_eq!(border.color, TestColor::Black);
        }
        _ => panic!("Expected DrawRectangle command"),
    }
}

/// Tests padding affecting content positioning within styled containers.
/// Verifies that padding correctly offsets content when combined with other styling.
#[test]
fn padding_with_styling() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(200.0), fixed!(100.0)))
                .padding(Padding::all(20.0))
                .background_color(TestColor::Red),
            |k| k.text("Padded", TextStyle::new()),
        )
    });

    assert_size!(commands.next(), (200.0, 100.0)); // Container
    assert_position!(commands.next(), (20.0, 20.0)); // Text offset by padding
}

/// Tests asymmetric padding with different values for each side.
/// Ensures that individual padding values are correctly applied.
#[test]
fn asymmetric_padding() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(400.0), fixed!(300.0)))
                .padding(Padding::new(10.0, 20.0, 15.0, 25.0)), // left, right, top, bottom
            |k| k.text("Asymmetric", TextStyle::new()),
        )
    });

    assert_size!(commands.next(), (400.0, 300.0)); // Container
    assert_position!(commands.next(), (10.0, 15.0)); // Text offset by left and top padding
}

/// Tests padding in different layout directions.
/// Verifies that padding works correctly with TopToBottom layout direction.
#[test]
fn padding_with_vertical_layout() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(200.0), fixed!(300.0)))
                .padding(Padding::all(30.0))
                .layout(Layout::new().direction(Direction::TopToBottom)),
            |k| {
                k.text("First", TextStyle::new())
                    .text("Second", TextStyle::new())
            },
        )
    });

    assert_size!(commands.next(), (200.0, 300.0)); // Container
    assert_position!(commands.next(), (30.0, 30.0)); // First text
    assert_position!(commands.next(), (30.0, 50.0)); // Second text (30 + 20 text height)
}

/// Tests styling inheritance with nested containers.
/// Verifies that styling properties don't interfere with color inheritance.
#[test]
fn styling_with_color_inheritance() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let mut commands = kaolin.draw(|k| {
        k.with(
            FlexStyle::new()
                .sizing(sizing!(fixed!(300.0), fixed!(200.0)))
                .color(TestColor::Red)
                .background_color(TestColor::Black)
                .border(Border::new().width(2.0).color(TestColor::Red)),
            |k| k.text("Styled Text", TextStyle::new()),
        )
    });

    commands.next(); // Container
    assert_color!(commands.next(), TestColor::Red); // Text should inherit red color
}

use kaolin::style::layout::{Direction, Layout};
