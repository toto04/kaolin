extern crate std;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    commands::RenderCommand,
    kaolin::{Kaolin, scope::KaolinScope},
    renderers::KaolinRenderer,
    style::{KaolinColor, TextStyle},
};
use raylib::{color::Color, prelude::*};

impl KaolinColor for Color {
    fn default_foreground_color() -> Self {
        Color::BLACK
    }

    fn default_background_color() -> Self {
        Color::WHITE
    }
}

pub struct RaylibRenderer {
    kaolin: Kaolin<Color>,
    raylib: Rc<RefCell<RaylibHandle>>,
    thread: RaylibThread,
}

impl RaylibRenderer {
    fn measure_text(
        this: Weak<RefCell<RaylibHandle>>,
        text: &str,
        config: &TextStyle<Color>,
    ) -> (f64, f64) {
        let raylib = this.upgrade().unwrap();
        let raylib = raylib.borrow();
        let len = raylib.measure_text(text, config.font_size as i32);
        (len as f64, config.font_size as f64)
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
            thread,
            raylib,
        }
    }

    pub fn should_close(&self) -> bool {
        let raylib = self.raylib.borrow();
        raylib.window_should_close()
    }
}

impl KaolinRenderer<Color> for RaylibRenderer {
    fn draw(&mut self, draw_fn: impl Fn(KaolinScope<Color>) -> KaolinScope<Color>) {
        let commands = self.kaolin.draw(draw_fn);
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
                    d.draw_rectangle(x as i32, y as i32, width as i32, height as i32, color);
                }
                RenderCommand::DrawText {
                    text,
                    x,
                    y,
                    color,
                    font_size,
                    ..
                } => {
                    d.draw_text(text.as_str(), x as i32, y as i32, font_size as i32, color);
                }
            }
        }
    }
}
