pub mod commands;
mod elements;
pub mod kaolin;
pub mod renderers;
pub mod style;

#[cfg(test)]
mod tests {
    use crate::{
        elements, grow,
        kaolin::Kaolin,
        style::{
            FlexStyle, flex_style,
            layout::{Alignment, Justification, Layout},
            sizing::*,
        },
    };

    #[test]
    fn simple_layout() {
        let mut kaolin = Kaolin::new((800, 600), |text, _| (text.len() as f32 * 10.0, 20.0));
        let commands = kaolin.draw(|k| {
            k.with(
                flex_style! {
                  sizing: BoxSizing {
                    width: grow!(1.0),
                    height: grow!(1.0),
                  },
                  layout: Layout {
                      justification: Justification::Center,
                      alignment: Alignment::Center,
                      direction: Default::default(),
                      gap: 0.0,
                  }
                },
                |k| k.text("Hello, Kaolin!", elements::text::TextConfig::default()),
            )
        });

        println!("{commands:?}");
        assert_eq!(commands.len(), 2);
    }
}
