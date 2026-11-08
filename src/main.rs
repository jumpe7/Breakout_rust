mod game;
mod player;
mod ball;

use macroquad::prelude::*;
use game::Game;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Breakout"),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut game = game::Game::new();
    loop {
        clear_background(BLACK);

        game.update();
        game.draw();

        next_frame().await;
    }
}