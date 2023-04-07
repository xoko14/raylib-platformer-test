use raylib::prelude::{Color, RaylibDraw, Rectangle};

use crate::engine::entities::{Collidable, Entity};

pub struct StagePart {
    pub rectangle: Rectangle,
    pub color: Color,
}

impl StagePart {
    pub fn new(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        rl: &mut raylib::RaylibHandle,
        thread: &raylib::RaylibThread,
    ) -> Self {
        let mut part = StagePart::load(rl, thread);
        part.rectangle.x = x;
        part.rectangle.y = y;
        part.rectangle.width = w;
        part.rectangle.height = h;
        part
    }
}

impl Collidable for StagePart {
    fn get_collider(&self) -> Rectangle {
        self.rectangle
    }
}

impl Entity for StagePart {
    fn load(_rl: &mut raylib::RaylibHandle, _thread: &raylib::RaylibThread) -> Self {
        Self {
            rectangle: Rectangle::default(),
            color: Color::BLACK,
        }
    }

    fn update(&mut self, _rl: &mut raylib::RaylibHandle, _: &Vec<impl Collidable>) {}

    fn draw(&self, d: &mut raylib::prelude::RaylibDrawHandle) {
        d.draw_rectangle_rec(&self.rectangle, &self.color);
    }
}
