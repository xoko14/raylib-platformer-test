use engine::entities::Entity;
use entities::{player::Player, stage::StagePart};
use raylib::prelude::{Color, RaylibDraw};

mod engine;
mod entities;

macro_rules! dt {
    ($rl: expr) => {
        $rl.get_frame_time() * 1000f32
    };
}
pub(crate) use dt;

#[tokio::main]
async fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello world").build();

    let mut player = Player::load(&mut rl, &thread);

    let level: Vec<StagePart> = vec![
        StagePart::new(0f32, 400f32, 640f32, 80f32, &mut rl, &thread),
        StagePart::new(500f32, 300f32, 100f32, 50f32, &mut rl, &thread),
    ];

    while !rl.window_should_close() {
        player.update(&mut rl, &level);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::CYAN);

        for part in &level {
            part.draw(&mut d);
        }

        player.draw(&mut d);
    }
}
