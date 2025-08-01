use std::rc::Rc;

use kaolin::{
    commands::RenderCommand,
    fixed, flex_style, grow,
    kaolin::Kaolin,
    layout, sizing,
    style::{
        FlexStyle, KaolinColor, TextConfig,
        layout::{Alignment, Justification},
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum TestColor {
    #[default]
    Black,
    Transparent,
}

impl KaolinColor<TestColor> for TestColor {
    fn default_foreground_color() -> Self {
        TestColor::Black
    }

    fn default_background_color() -> Self {
        TestColor::Transparent
    }
}

fn measure_text(text: &str, _config: &TextConfig<TestColor>) -> (f32, f32) {
    (text.len() as f32 * 10.0, 20.0)
}

#[test]
fn simple_layout() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.with(
            flex_style! {
              sizing: sizing! {
                  width: grow!(1.0),
                  height: grow!(1.0),
              },
              layout: layout! {
                  justification: Justification::Center,
                  alignment: Alignment::Center,
              }
            },
            |k| k.text("Hello, Kaolin!", TextConfig::default()),
        )
    });

    println!("{commands:?}");
    assert_eq!(commands.len(), 2);
    assert_eq!(
        commands.collect::<Vec<_>>(),
        vec![
            RenderCommand::DrawRectangle {
                id: "".to_string(), // ID is not used in this context
                color: TestColor::Transparent,
                x: 0,
                y: 0,
                width: 800,
                height: 600
            },
            RenderCommand::DrawText {
                config: Rc::new(TextConfig::default()),
                text: "Hello, Kaolin!".to_string(),
                x: (800 - 140) / 2, // Assuming 10px per character width
                y: (600 - 20) / 2,  // Assuming 20px per line height
            },
        ]
    );
}

#[test]
fn double_growth() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.with(
            flex_style! {
              sizing: sizing! {
                width: grow!(1.0),
                height: grow!(1.0),
              }
            },
            |k| k,
        )
        .with(
            flex_style! {
                sizing: sizing! {
                    width: grow!(3.0),
                    height: fixed!(200.0),
                }
            },
            |k| k,
        )
    });

    println!("{commands:?}");
    assert_eq!(commands.len(), 2);
    assert_eq!(
        commands.collect::<Vec<_>>(),
        vec![
            RenderCommand::DrawRectangle {
                id: "".to_string(), // ID is not used in this context
                color: TestColor::Transparent,
                x: 0,
                y: 0,
                width: 200,
                height: 600
            },
            RenderCommand::DrawRectangle {
                id: "".to_string(), // ID is not used in this context
                color: TestColor::Transparent,
                x: 200,
                y: 0,
                width: 600,
                height: 200
            },
        ]
    );
}

#[test]
fn fit_sizing() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    let commands = kaolin.draw(|k| {
        k.with(flex_style! {}, |k| {
            k.text("Hello, Kaolin!", TextConfig::default())
        })
    });

    println!("{commands:?}");
    assert_eq!(commands.len(), 2);
    assert_eq!(
        commands.collect::<Vec<_>>(),
        vec![
            RenderCommand::DrawRectangle {
                id: "".to_string(), // ID is not used in this context
                color: TestColor::Transparent,
                x: 0,
                y: 0,
                width: 140,
                height: 20
            },
            RenderCommand::DrawText {
                config: Rc::new(TextConfig::default()),
                text: "Hello, Kaolin!".to_string(),
                x: 0,
                y: 0
            }
        ]
    );
}
