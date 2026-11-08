use macroquad::prelude::*;
use crate::ball::Ball;
use crate::player::Player;

pub struct Game{
    player: Player,
    ball: Ball,
}

impl Game{
    pub fn new() -> Self {
        Self{
            player: Player::new(),
            ball: Ball::new(),
        }
    }
    pub fn update(&mut self) {
        let dt = get_frame_time();
        self.player.update(dt);
        self.ball.update(dt);
        let player_rect = self.player.rect();
        let ball_rect = self.ball.rect();
        if player_rect.overlaps(&ball_rect){
            self.ball.update_player(player_rect);
        }
    }

    pub fn draw(&self){
        self.player.draw();
        self.ball.draw();
    }
}