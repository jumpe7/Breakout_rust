use macroquad::prelude::*;

pub struct Player {
    position: Vec2,
    size: Vec2,
    speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Self{
            position: Vec2::new(screen_width()/2.0, screen_height()-100.0),
            size: Vec2::new(125.0,15.0),
            speed: 400.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.position.x -= self.speed*dt;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.position.x += self.speed*dt;
        }

        if self.position.x + self.size.x/2.0 > screen_width() {
            self.position.x = screen_width()-self.size.x/2.0;
        }
        if self.position.x - self.size.x/2.0 < 0.0 {
            self.position.x = 0.0+self.size.x/2.0;
        }
    }

    pub fn rect(&self) -> Rect{
        let x = self.position.x - self.size.x/2.0;
        let y = self.position.y - self.size.y/2.0;
        Rect::new(x,y, self.size.x, self.size.y)
    }

    pub fn draw(&self){
        draw_rectangle(self.rect().x, self.rect().y, self.size.x, self.size.y, WHITE);
    }
}