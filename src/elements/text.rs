use unicode_segmentation::UnicodeSegmentation;

use crate::{commands::RenderCommand, kaolin::MeasureTextFn, style::colors::Color};

#[derive(Debug, Clone, Copy)]
pub struct TextConfig {
    pub font_id: u32,
    pub font_size: f32,
    pub color: Color,
}

impl Default for TextConfig {
    fn default() -> Self {
        TextConfig {
            font_id: 0,
            font_size: 16.0,
            color: Color::default(),
        }
    }
}

pub struct TextElement {
    content: String,
    config: TextConfig,
    measure_text: MeasureTextFn,
    lines: Vec<(usize, usize)>, // (start, end) indices of lines in content
}

impl TextElement {
    pub fn new(content: &str, config: TextConfig, measure_text: MeasureTextFn) -> Self {
        let lines = Vec::new();
        TextElement {
            content: content.to_string(),
            config,
            measure_text,
            lines,
        }
    }

    pub fn get_preferred_size(&self) -> (f32, f32) {
        self.content
            .lines()
            .fold((0.0, 0.0), |(max_width, total_height), line| {
                let (width, height) = (self.measure_text)(line, &self.config);
                (max_width.max(width), total_height + height)
            })
    }

    pub fn get_minimum_size(&self) -> (f32, f32) {
        self.content
            .split_whitespace()
            .fold((0.0, 0.0), |(min_width, min_height), word| {
                let (width, height) = (self.measure_text)(word, &self.config);
                (min_width.max(width), min_height.max(height))
            })
    }

    pub fn wrap_text(&mut self, current_width: f32) -> f32 {
        // what we should actually do is find the positions of newlines
        // for each line either split at the last whitespace if the line is too long or at the newline
        // save all the absolute positions of the splitpoints for rendering later

        let mut newlines = self.content.match_indices('\n').map(|(i, _)| i);
        let mut nextline = newlines.next().unwrap_or(self.content.len());
        let word_indices = self.content.unicode_word_indices().skip(1).map(|(i, _)| i);
        let mut start = 0;
        let mut prev_last = 0;
        let mut total_height = 0.0;
        for next_word_start in word_indices {
            if next_word_start >= nextline {
                self.lines.push((start, nextline));
                start = next_word_start;
                prev_last = next_word_start;
                nextline = newlines.next().unwrap_or(self.content.len());
            } else {
                let slice = &self.content[start..next_word_start].trim_end();
                let (width, height) = (self.measure_text)(slice, &self.config);

                if width > current_width {
                    self.lines.push((start, prev_last));
                    total_height += height;
                    start = next_word_start;
                }
                prev_last = start + slice.len();
            }
        }
        let last_slice = &self.content[start..].trim_end();
        self.lines.push((start, last_slice.len() + start));
        if !last_slice.is_empty() {
            let (_, height) = (self.measure_text)(last_slice, &self.config);
            total_height += height;
        }
        total_height
    }

    pub fn render<'a>(&'a self, x: f32, y: f32) -> Vec<RenderCommand<'a>> {
        let mut commands = Vec::new();
        let mut current_y = y;
        for line_indices in self.lines.iter() {
            let (start, end) = *line_indices;
            let line = &self.content[start..end];
            let (_, height) = (self.measure_text)(line, &self.config);
            commands.push(RenderCommand::DrawText {
                text: line,
                x: x as i32,
                y: current_y as i32,
                config: &self.config,
            });
            current_y += height;
        }
        commands
    }
}
