use macroquad::prelude::*;

use crate::paddle::Paddle;

pub struct Ball {
    pub position: Vec2,
    pub direction: Vec2,
    pub size: Vec2,
}

impl Ball {
    pub fn new(position: Vec2, direction: Vec2, size: Vec2) -> Ball {
        Ball {
            position,
            direction,
            size,
        }
    }
    
    pub fn update(&mut self, speed: f32, left_paddle: &Paddle, right_paddle: &Paddle) {
        // Horizontal
        if self.colliding(left_paddle) {
            self.direction.x = 1.0;
        }
        if self.colliding(right_paddle) {
            self.direction.x = -1.0;
        }

        //vertical ( asegurando bordes del mapa)
        if self.position.y <= 0.0 {
            self.direction.y = 1.0;
        }
        if self.position.y >= screen_height() - self.size.y {
            self.direction.y = -1.0;
        }

        self.position += self.direction * speed;
    }
    
    pub fn colliding(&self, paddle: &Paddle) -> bool {
        return
            (self.position.x + self.size.x > paddle.position.x && self.position.x < paddle.position.x + paddle.size.x) &&
            (self.position.y + self.size.y > paddle.position.y && self.position.y < paddle.position.y + paddle.size.y);
    }
    
    pub fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, WHITE);
    }
}