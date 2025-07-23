use std::cell::RefCell;

use crate::{commands::RenderCommand, renderers::KaolinRenderer};
use raylib::prelude::*;

pub struct RaylibRenderer {
    raylib: RefCell<RaylibHandle>,
    thread: RaylibThread,
}

impl RaylibRenderer {
    pub fn new(width: i32, height: i32) -> Self {
        let (raylib, thread) = raylib::init()
            .size(width, height)
            .title("Kaolin Renderer")
            .build();
        RaylibRenderer {
            raylib: RefCell::new(raylib),
            thread,
        }
    }

    pub fn draw_fn<F>(mut draw_fn: F)
    where
        F: FnMut(&mut RaylibRenderer),
    {
        let mut rr = RaylibRenderer::new(800, 600);
        while !rr.raylib.borrow().window_should_close() {
            draw_fn(&mut rr);
        }
    }
}

impl KaolinRenderer for RaylibRenderer {
    fn draw(&mut self, commands: Vec<RenderCommand>) {
        let mut raylib = self.raylib.borrow_mut();
        let mut d = raylib.begin_drawing(&self.thread);
        for command in commands {
            match command {
                RenderCommand::DrawRectangle {
                    x,
                    y,
                    width,
                    height,
                    color,
                    ..
                } => {
                    let color = raylib::color::Color::from(color.rgba());
                    d.draw_rectangle(x, y, width, height, color);
                }
                RenderCommand::DrawText { text, x, y, config } => {
                    let color = raylib::color::Color::from(config.color.rgba());
                    d.draw_text(text, x, y, config.font_size as i32, color);
                }
            }
        }
    }
}
