use kaolin::{Kaolin, fixed, sizing, style::FlexStyle};

mod common;
use common::*;

/// Tests error handling for invalid configurations.
/// Ensures that the system panics or handles errors gracefully when invalid properties are set.
#[test]
#[should_panic]
fn invalid_sizing_configuration() {
    let kaolin = Kaolin::new((800, 600), measure_text);
    kaolin.draw(|k| {
        k.with(
            FlexStyle::new().sizing(sizing!(fixed!(-100.0), fixed!(50.0))),
            |k| k,
        )
    });
}
