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
use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
struct KRaylibColor {
    inner: raylib::color::Color,
}

impl From<KRaylibColor> for raylib::color::Color {
    fn from(value: KRaylibColor) -> Self {
        value.inner
    }
}

impl From<raylib::color::Color> for KRaylibColor {
    fn from(value: raylib::color::Color) -> Self {
        KRaylibColor { inner: value }
    }
}

impl KaolinColor<KRaylibColor> for KRaylibColor {
    fn default_foreground_color() -> KRaylibColor {
        KRaylibColor {
            inner: raylib::color::Color::BLACK,
        }
    }

    fn default_background_color() -> KRaylibColor {
        KRaylibColor {
            inner: raylib::color::Color::WHITE,
        }
    }
}

pub struct RaylibRenderer {
    kaolin: Kaolin<KRaylibColor>,
    raylib: Rc<RefCell<RaylibHandle>>,
    thread: RaylibThread,
}

impl RaylibRenderer {
    fn measure_text(
        this: Weak<RefCell<RaylibHandle>>,
        text: &str,
        config: &TextStyle<KRaylibColor>,
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

impl KaolinRenderer<KRaylibColor> for RaylibRenderer {
    fn draw(&mut self, draw_fn: impl Fn(KaolinScope<KRaylibColor>) -> KaolinScope<KRaylibColor>) {
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
                    d.draw_rectangle(x as i32, y as i32, width as i32, height as i32, color.inner);
                }
                RenderCommand::DrawText {
                    text,
                    x,
                    y,
                    color,
                    font_size,
                    ..
                } => {
                    d.draw_text(
                        text.as_str(),
                        x as i32,
                        y as i32,
                        font_size as i32,
                        color.inner,
                    );
                }
            }
        }
    }
}
