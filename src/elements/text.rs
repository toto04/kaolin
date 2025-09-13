use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use typed_floats::tf64::Positive;
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    commands::RenderCommand,
    elements::traits::{KaolinContainerElement, KaolinElement},
    kaolin::MeasureTextFnRef,
    style::{
        TextStyle,
        sizing::{PreferredSize, SizingDimensions},
    },
};

/// Represents a text element in the UI.
pub(crate) struct TextElement<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    content: String,
    style: TextStyle<Color>,
    lines: Vec<(usize, usize)>, // (start, end) indices of lines in content
    measure_text: MeasureTextFnRef<Color>,
    inherited_color: Option<Color>, // measure text function
}

impl<Color> TextElement<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    pub fn new(
        content: &str,
        style: TextStyle<Color>,
        measure_text: MeasureTextFnRef<Color>,
    ) -> Self {
        let lines = Vec::new();
        TextElement {
            content: content.to_string(),
            style,
            lines,
            measure_text,
            inherited_color: None,
        }
    }

    fn measure_text(&self, text: &str) -> (f64, f64) {
        if let Some(measure_text) = self.measure_text.upgrade() {
            (measure_text)(text, &self.style)
        } else {
            (0.0, 0.0)
        }
    }

    /// Calculates the preferred size of the text element, without wrapping.
    pub fn get_preferred_size(
        &self,
    ) -> (
        typed_floats::tf64::PositiveFinite,
        typed_floats::tf64::PositiveFinite,
    ) {
        let floats =
            self.content
                .lines()
                .fold((0.0f64, 0.0f64), |(max_width, total_height), line| {
                    let (width, height) = self.measure_text(line);
                    (max_width.max(width), total_height + height)
                });
        (
            typed_floats::tf64::PositiveFinite::new(floats.0).unwrap(),
            typed_floats::tf64::PositiveFinite::new(floats.1).unwrap(),
        )
    }

    /// Calculates the minimum size the text can wrap to without overflowing.
    pub fn get_minimum_size(
        &self,
    ) -> (
        typed_floats::tf64::PositiveFinite,
        typed_floats::tf64::PositiveFinite,
    ) {
        let floats = self.content.split_whitespace().fold(
            (0.0f64, 0.0f64),
            |(min_width, min_height), word| {
                let (width, height) = self.measure_text(word);
                (min_width.max(width), min_height.max(height))
            },
        );
        (
            typed_floats::tf64::PositiveFinite::new(floats.0).unwrap(),
            typed_floats::tf64::PositiveFinite::new(floats.1).unwrap(),
        )
    }

    /// Wraps the text to fit within the specified width.
    pub fn wrap_text(&mut self, current_width: f64) -> f64 {
        // what we should actually do is find the positions of newlines
        // for each line either split at the last whitespace if the line is too long or at the newline
        // save all the absolute positions of the splitpoints for rendering later

        let mut newlines = self.content.match_indices('\n').map(|(i, _)| i);
        let mut maybe_newline = newlines.next();
        let word_indices = self
            .content
            .unicode_word_indices()
            .skip(1) // the first word has to be in the first line duh
            .map(|(i, _)| i)
            .chain(core::iter::once(self.content.len())); // end of the content as last "next word index"

        // the current start of the slice
        let mut start = 0;
        // the end of the previous word, if we wrap that's where we should end
        let mut prev_last = 0;
        // the start of the previous word, if we wrap that's the new start
        let mut prev_word_start = 0;
        // accumulated height
        let mut total_height = 0.0;
        for next_word_start in word_indices {
            if let Some(next_newline) = maybe_newline
                && next_word_start >= next_newline
            {
                self.lines.push((start, next_newline));
                start = next_word_start;
                prev_last = next_word_start;
                maybe_newline = newlines.next();
            } else {
                // if we wrap, we should check the new word again
                loop {
                    let slice = self.content[start..next_word_start].trim_end();
                    let (width, height) = self.measure_text(slice);

                    if start < prev_last && width > current_width {
                        // it's a wrap!
                        self.lines.push((start, prev_last)); // push the wrapped line
                        total_height += height; // accumulate height
                        start = prev_word_start; // reset start to the previous word start
                    } else {
                        // no wrap, set where we ended up with and go to next word
                        prev_last = start + slice.len();
                        break;
                    }
                }
            }
            // update the previous word start
            prev_word_start = next_word_start;
        }
        let last_slice = &self.content[start..].trim_end();
        if !last_slice.is_empty() {
            self.lines.push((start, last_slice.len() + start));
            let (_, height) = self.measure_text(last_slice);
            total_height += height;
        }
        total_height
    }
}

impl<'frame, Color, CustomData> KaolinElement<'frame, Color, CustomData> for TextElement<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor,
{
    fn get_sizing_dimensions(&self) -> (SizingDimensions, SizingDimensions) {
        let (pref_width, pref_height) = self.get_preferred_size();
        let width = SizingDimensions {
            min: self.get_minimum_size().0,
            preferred: PreferredSize::Fixed(pref_width),
            max: pref_width.into(),
        };
        let height = SizingDimensions {
            min: pref_height,
            preferred: PreferredSize::Fixed(pref_height),
            max: Positive::new(f64::INFINITY).unwrap(),
        };
        (width, height)
    }

    fn render(
        &self,
        offsets: (f64, f64),
        _size: (f64, f64),
    ) -> Box<dyn Iterator<Item = RenderCommand<Color, CustomData>> + '_> {
        let mut current_y = offsets.1;
        Box::new(self.lines.iter().filter_map(move |line_indices| {
            let (start, end) = *line_indices;
            let line = self.content[start..end].trim();
            if line.is_empty() {
                return None;
            }
            let (_, height) = self.measure_text(line);
            let y = current_y;
            current_y += height;

            Some(RenderCommand::DrawText {
                text: line.to_string(),
                x: offsets.0,
                y,
                font_id: self.style.font_id,
                font_size: self.style.font_size,
                color: self
                    .style
                    .color
                    .or(self.inherited_color)
                    .unwrap_or(Color::default_foreground_color()),
            })
        }))
    }

    fn default_growable_width(&self, _sizing: &crate::style::sizing::SizingDimensions) -> bool {
        false
    }

    fn default_shrinkable(&self, _sizing: &crate::style::sizing::SizingDimensions) -> bool {
        true
    }

    fn default_growable_height(&self, _sizing: &crate::style::sizing::SizingDimensions) -> bool {
        false
    }

    fn fit_height_unbound(&mut self, final_width: f64) -> f64 {
        self.wrap_text(final_width)
    }

    fn inherit_color(&mut self, inherited_color: Color) {
        self.inherited_color = Some(inherited_color);
    }

    fn as_container(
        &mut self,
    ) -> Option<&mut dyn KaolinContainerElement<'frame, Color, CustomData>> {
        None
    }
}
