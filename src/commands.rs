use std::{collections::VecDeque, rc::Rc};

use crate::{elements::flexbox::FlexBox, style::TextConfig};

#[derive(Debug, Clone)]
pub enum RenderCommand<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    DrawRectangle {
        id: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: Color,
    },
    DrawText {
        text: String,
        x: i32,
        y: i32,
        config: Rc<TextConfig<Color>>,
    },
}

impl<Color> PartialEq for RenderCommand<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                RenderCommand::DrawRectangle {
                    x,
                    y,
                    width,
                    height,
                    color,
                    ..
                },
                RenderCommand::DrawRectangle {
                    x: other_x,
                    y: other_y,
                    width: other_width,
                    height: other_height,
                    color: other_color,
                    ..
                },
            ) => {
                x == other_x
                    && y == other_y
                    && width == other_width
                    && height == other_height
                    && color == other_color
            }
            (
                RenderCommand::DrawText { text, x, y, config },
                RenderCommand::DrawText {
                    text: other_text,
                    x: other_x,
                    y: other_y,
                    config: other_config,
                },
            ) => text == other_text && x == other_x && y == other_y && config == other_config,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RenderCommands<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    commands: VecDeque<RenderCommand<Color>>,
}

impl<Color> RenderCommands<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    pub fn new(root: FlexBox<Color>) -> Self {
        let children = root.children;
        RenderCommands {
            commands: children.render_nodes().collect::<VecDeque<_>>(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn len(&self) -> usize {
        self.commands.len()
    }
}

impl<Color> Iterator for RenderCommands<Color>
where
    Color: Default + Copy + PartialEq + crate::style::KaolinColor<Color>,
{
    type Item = RenderCommand<Color>;

    fn next(&mut self) -> Option<Self::Item> {
        self.commands.pop_front()
    }
}
