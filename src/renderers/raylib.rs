use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    commands::RenderCommand,
    kaolin::{Kaolin, scope::KaolinScope},
    renderers::KaolinRenderer,
    style::TextConfig,
};
use raylib::prelude::*;

pub struct RaylibRenderer<'frame> {
    kaolin: Kaolin<'frame>,
    raylib: Rc<RefCell<RaylibHandle>>,
    thread: RaylibThread,
}

impl<'frame> RaylibRenderer<'frame> {
    fn measure_text(
        this: Weak<RefCell<RaylibHandle>>,
        text: &str,
        config: &TextConfig,
    ) -> (f32, f32) {
        let raylib = this.upgrade().unwrap();
        let raylib = raylib.borrow();
        let len = raylib.measure_text(text, config.font_size as i32);
        (len as f32, config.font_size)
    }

    pub fn new(width: i32, height: i32) -> Self {
        let (raylib_handle, thread) = raylib::init()
            .size(width, height)
            .title("Kaolin Renderer")
            .build();
        let raylib = Rc::new(RefCell::new(raylib_handle));
        let weakref = Rc::downgrade(&raylib);

        RaylibRenderer {
            kaolin: Kaolin::new((width, height), move |text, config| {
                RaylibRenderer::measure_text(weakref.clone(), text, config)
            }),
            raylib,
            thread,
        }
    }

    pub fn should_close(&self) -> bool {
        let raylib = self.raylib.borrow();
        raylib.window_should_close()
    }
}

impl KaolinRenderer for RaylibRenderer<'_> {
    fn draw(&mut self, drawing_fn: fn(KaolinScope) -> KaolinScope) {
        let commands = self.kaolin.draw(drawing_fn);
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
                    d.draw_text(text.as_str(), x, y, config.font_size as i32, color);
                }
            }
        }
    }
}
