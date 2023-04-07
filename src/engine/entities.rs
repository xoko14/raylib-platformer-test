use raylib::{
    prelude::{RaylibDrawHandle, Rectangle},
    RaylibHandle, RaylibThread,
};

pub trait Entity {
    fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self;
    fn update(&mut self, rl: &mut RaylibHandle, level: &Vec<impl Collidable>);
    fn draw(&self, d: &mut RaylibDrawHandle);
}

pub trait Collidable {
    fn get_collider(&self) -> Rectangle;
}
