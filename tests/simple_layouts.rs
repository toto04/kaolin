use kaolin::{
    Kaolin,
    commands::RenderCommand,
    fixed, grow, sizing,
    style::{
        FlexStyle, KaolinColor, TextStyle,
        border::Border,
        layout::{Alignment, Direction, Justification, Layout},
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum TestColor {
    #[default]
    Black,
    Transparent,
    Red,
}

impl KaolinColor<TestColor> for TestColor {
    fn default_foreground_color() -> Self {
        TestColor::Black
    }

    fn default_background_color() -> Self {
        TestColor::Transparent
    }
}

fn measure_text(text: &str, _config: &TextStyle<TestColor>) -> (f32, f32) {
    (text.len() as f32 * 10.0, 20.0)
}

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
                x: 0,
                y: 0,
                width: 800,
                height: 600,
                corner_radius: 0.0,
                border: Border::default()
            },
            RenderCommand::DrawText {
                text: "Hello, Kaolin!".to_string(),
                x: (800 - 140) / 2,
                y: (600 - 20) / 2,
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
                x: 0,
                y: 0,
                width: 200,
                height: 600,
                corner_radius: 0.0,
                border: Border::default(),
            },
            RenderCommand::DrawRectangle {
                id: "".to_string(), // ID is not used in this context
                color: TestColor::Transparent,
                x: 200,
                y: 0,
                width: 600,
                height: 200,
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
                x: 0,
                y: 0,
                width: 140,
                height: 20,
                corner_radius: 0.0,
                border: Border::default(),
            },
            RenderCommand::DrawText {
                text: "Hello, Kaolin!".to_string(),
                x: 0,
                y: 0,
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
                x: 0,
                y: 0,
                width: 140,
                height: 20,
                corner_radius: 0.0,
                border: Border::default(),
            },
            RenderCommand::DrawText {
                text: "Hello, Kaolin!".to_string(),
                x: 0,
                y: 0,
                font_id: 0,
                font_size: 16.0,
                color: TestColor::Red, // Inherited color
            },
        ]
    );
}
