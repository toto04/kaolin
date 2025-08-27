#![feature(test)]
extern crate test;

use kaolin::{
    Kaolin, fit, fixed, grow, sizing,
    style::{FlexStyle, TextStyle},
};
use test::{Bencher, black_box};

mod common;
use common::*;

#[bench]
fn bench_text_wrapping(b: &mut Bencher) {
    let kaolin = Kaolin::new((800, 600), measure_text);
    b.iter(|| {
        let commands = kaolin.draw(|k| {
            k.with(
                FlexStyle::new().sizing(sizing!(fixed!(100.0), fit!())),
                |k| k.text("This is a long text that should wrap", TextStyle::new()),
            )
            .with(FlexStyle::new().sizing(sizing!(grow!())), |k| {
                k.text("This is a long text that should wrap", TextStyle::new())
            })
            .with(FlexStyle::new().sizing(sizing!(grow!(2.0))), |k| {
                k.text("This is a long text that should wrap", TextStyle::new())
            })
        });
        black_box(commands.collect::<Vec<_>>());
    });
}

mod clay_bench {
    extern crate clay_layout;
    extern crate test;
    use test::{Bencher, black_box};

    use clay_layout::{Clay, Declaration, fixed, grow, math::Dimensions, text::TextConfig};

    pub fn measure_text(text: &str, _config: &TextConfig) -> Dimensions {
        (text.len() as f32 * 10.0, 20.0).into()
    }

    #[bench]
    fn clay_same_layout(b: &mut Bencher) {
        // Create the clay instance
        let mut clay: Clay = Clay::new((800., 600.).into());
        clay.set_measure_text_function(measure_text);
        b.iter(|| {
            // Begin the layout
            let mut clay = clay.begin::<(), ()>();

            // Adds a red rectangle with a corner radius of 5.
            // The Layout makes the rectangle have a width and height of 50.
            clay.with(Declaration::new().layout().width(fixed!(50.)).end(), |c| {
                c.text(
                    "This is a long text that should wrap",
                    TextConfig::new().end(),
                )
            });
            clay.with(
                Declaration::new()
                    .layout()
                    .width(grow!(1.0))
                    .height(grow!(1.0))
                    .end(),
                |c| {
                    c.text(
                        "This is a long text that should wrap",
                        TextConfig::new().end(),
                    )
                },
            );
            clay.with(
                Declaration::new()
                    .layout()
                    .width(grow!(2.0))
                    .height(grow!(2.0))
                    .end(),
                |c| {
                    c.text(
                        "This is a long text that should wrap",
                        TextConfig::new().end(),
                    )
                },
            );

            black_box(clay.end().collect::<Vec<_>>());
        });
    }
}
