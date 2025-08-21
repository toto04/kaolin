use std::iter;

use unicode_segmentation::UnicodeSegmentation;

use crate::{commands::RenderCommand, kaolin::MeasureTextFn, style::TextStyle};

/// Represents a text element in the UI.
pub struct TextElement<'frame, Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    content: String,
    style: TextStyle<Color>,
    lines: Vec<(usize, usize)>, // (start, end) indices of lines in content
    measure_text: &'frame MeasureTextFn<'frame, Color>, // measure text function
}

impl<'frame, Color> TextElement<'frame, Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    pub fn new(
        content: &str,
        style: TextStyle<Color>,
        measure_text: &'frame MeasureTextFn<'frame, Color>,
    ) -> Self {
        let lines = Vec::new();
        TextElement {
            content: content.to_string(),
            style,
            lines,
            measure_text,
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
                    let (width, height) = (self.measure_text)(line, &self.style);
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
                let (width, height) = (self.measure_text)(word, &self.style);
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
        let mut nextline = newlines.next().unwrap_or(self.content.len() + 1);
        let word_indices = self
            .content
            .unicode_word_indices()
            .skip(1)
            .map(|(i, _)| i)
            .chain(iter::once(self.content.len()));

        // the current start of the slice
        let mut start = 0;
        // the end of the previous word, if we wrap that's where we should end
        let mut prev_last = 0;
        // the start of the previous word, if we wrap that's the new start
        let mut prev_word_start = 0;
        // accumulated height
        let mut total_height = 0.0;
        for next_word_start in word_indices {
            if next_word_start >= nextline {
                self.lines.push((start, nextline));
                start = next_word_start;
                prev_last = next_word_start;
                nextline = newlines.next().unwrap_or(self.content.len());
            } else {
                // if we wrap, we should check the new word again
                loop {
                    let slice = self.content[start..next_word_start].trim_end();
                    let (width, height) = (self.measure_text)(slice, &self.style);

                    if width > current_width {
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
        self.lines.push((start, last_slice.len() + start));
        if !last_slice.is_empty() {
            let (_, height) = (self.measure_text)(last_slice, &self.style);
            total_height += height;
        }
        total_height
    }

    /// Renders the text element, returning an iterator of rendering commands,
    /// one for each line of text.
    pub fn render(
        &self,
        x: f64,
        y: f64,
        inherited_color: Color,
    ) -> impl Iterator<Item = RenderCommand<Color>> {
        let mut current_y = y;
        self.lines.iter().map(move |line_indices| {
            let (start, end) = *line_indices;
            let line = &self.content[start..end];
            let (_, height) = (self.measure_text)(line, &self.style);
            let y = current_y;
            current_y += height;

            RenderCommand::DrawText {
                text: line.to_string(),
                x,
                y,
                font_id: self.style.font_id,
                font_size: self.style.font_size,
                color: self.style.color.unwrap_or(inherited_color),
            }
        })
    }
}
