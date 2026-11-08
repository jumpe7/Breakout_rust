use macroquad::prelude::*;

pub struct Ball{
    position: Vec2,
    radius: f32,
    velocity: Vec2,
}

impl Ball{
    pub fn new() -> Self{
        Self{
            position: Vec2::new(screen_width()/2.0, screen_height()-150.0),
            radius: 10.0,
            velocity: Vec2::new(250.0,-250.0),
        }
    }

    pub fn update(&mut self, dt:f32){
        self.position += self.velocity * dt;

        if self.position.x-self.radius<=0.0{
            self.position.x = self.radius;
            self.velocity.x = -self.velocity.x;
        }
        if self.position.y-self.radius<=0.0{
            self.position.y = self.radius;
            self.velocity.y = -self.velocity.y;
        }
        if self.position.x+self.radius >= screen_width(){
            self.position.x = screen_width() - self.radius;
            self.velocity.x = -self.velocity.x;
        }
    }

    pub fn update_player(&mut self, player_rect: Rect){
        if self.velocity.y > 0.0 {
            let offset = (
                (self.position.x - (player_rect.x + player_rect.w / 2.0))
                    / (player_rect.w / 2.0)
            ).clamp(-1.0, 1.0);

            let max_angle = std::f32::consts::FRAC_PI_3;
            let angle = offset * max_angle;

            let speed = self.velocity.length();

            self.velocity.x = speed * angle.sin();
            self.velocity.y = -speed * angle.cos();

            self.position.y = player_rect.y - self.radius;
        }
    }

    pub fn rect(&self) -> Rect{
        let x = self.position.x - self.radius;
        let y = self.position.y - self.radius;
        Rect::new(x,y,self.radius*2.0,self.radius*2.0)
    }

    pub fn draw(&self){
        draw_circle(
            self.position.x,
            self.position.y,
            self.radius,
            WHITE
        )
    }
}