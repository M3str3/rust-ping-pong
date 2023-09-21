use macroquad::prelude::*;
pub struct Paddle {
    pub position: Vec2,
    pub size: Vec2,
}

impl Paddle {
    pub fn new(position: Vec2, size: Vec2) -> Paddle {
        Paddle {
            position,
            size,
        }
    }
    
    pub fn update(&mut self, speed: f32, up_key: KeyCode, down_key: KeyCode) {
        if is_key_down(up_key) {
            self.position.y -= speed;
        }
        if is_key_down(down_key) {
            self.position.y += speed;
        }
        self.position.y = self.position.y.clamp(0.0, screen_height() - self.size.y);
    }
    
    pub fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, WHITE);
    }
}
