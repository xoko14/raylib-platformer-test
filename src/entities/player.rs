use raylib::{
    prelude::{Color, KeyboardKey, RaylibDraw, Rectangle, Vector2},
    texture::Texture2D,
};

use crate::{
    dt,
    engine::entities::{Collidable, Entity},
};

static GRAVITY: f32 = 0.4f32;
static PLAYER_SPEED: f32 = 0.6f32;
static JUMP_FORCE: f32 = 0.5f32;
static MAX_JUMP: f32 = 150f32;

pub struct Player {
    pub position: Vector2,
    texture: Texture2D,
    size: Vector2,
    is_grounded: bool,
    vertical_force: f32,
    jump_height: f32,
}

impl Player {
    fn get_collider(&self) -> Rectangle {
        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: self.size.x,
            height: self.size.y,
        }
    }
}

impl Entity for Player {
    fn load(rl: &mut raylib::RaylibHandle, thread: &raylib::RaylibThread) -> Self {
        Self {
            position: Vector2::default(),
            texture: rl
                .load_texture(thread, "./assets/sprites/player.png")
                .unwrap(),
            size: Vector2 { x: 32f32, y: 32f32 },
            is_grounded: false,
            vertical_force: 0f32,
            jump_height: 0f32,
        }
    }

    fn update(&mut self, rl: &mut raylib::RaylibHandle, level: &Vec<impl Collidable>) {
        let current_area = self.get_collider();

        for col in level {
            if col.get_collider().check_collision_recs(&current_area) {
                self.is_grounded = true;
                break;
            }
            self.is_grounded = false;
        }

        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.position.x += PLAYER_SPEED * dt!(rl);
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            self.position.x -= PLAYER_SPEED * dt!(rl);
        }
        if self.is_grounded {
            self.vertical_force = 0f32;
            self.jump_height = 0f32;

            if rl.is_key_pressed(KeyboardKey::KEY_UP) {
                self.jump_height += JUMP_FORCE * dt!(rl);
                self.vertical_force = -JUMP_FORCE;
            }
        } else {
            if rl.is_key_down(KeyboardKey::KEY_UP) && self.jump_height < MAX_JUMP {
                self.jump_height += JUMP_FORCE * dt!(rl);
                self.vertical_force = -JUMP_FORCE;
            } else {
                self.jump_height = MAX_JUMP;
                self.vertical_force = GRAVITY;
            }
        }

        self.position.y += self.vertical_force * dt!(rl);
    }

    fn draw(&self, d: &mut raylib::prelude::RaylibDrawHandle) {
        d.draw_texture_v(&self.texture, self.position, Color::WHITE)
    }
}
