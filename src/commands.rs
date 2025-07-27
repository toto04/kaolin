use std::{collections::VecDeque, rc::Rc};

use crate::{
    elements::flexbox::FlexBox,
    style::{TextConfig, colors::Color},
};

#[derive(Debug, Clone)]
pub enum RenderCommand {
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
        config: Rc<TextConfig>,
    },
}

impl PartialEq for RenderCommand {
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
pub struct RenderCommands {
    commands: VecDeque<RenderCommand>,
}

impl RenderCommands {
    pub fn new(root: FlexBox) -> Self {
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

impl Iterator for RenderCommands {
    type Item = RenderCommand;

    fn next(&mut self) -> Option<Self::Item> {
        self.commands.pop_front()
    }
}
