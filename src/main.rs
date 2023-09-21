use macroquad::prelude::*;
use macroquad::rand::gen_range;

mod paddle;
use paddle::Paddle;

mod ball;
use ball::Ball;

fn window_config() -> Conf {
    /* Window configuration */
    Conf {
        window_title: "Pong".to_string(),
        ..Default::default()
    }
}

// To restart ball, left & right paddle 
fn reset_game(paddle_size: Vec2, ball_size: Vec2) -> (Paddle, Paddle, Ball) {
    let left_paddle = Paddle::new(vec2(10.0, (screen_height() - paddle_size.y) / 2.0), paddle_size);
    let right_paddle = Paddle::new(vec2(screen_width() - 10.0 - paddle_size.x, (screen_height() - paddle_size.y) / 2.0), paddle_size);
    let ball_direction = vec2(gen_range(0, 2) as f32, gen_range(0, 2) as f32) * 2.0 - 1.0;
    let ball = Ball::new((vec2(screen_width(), screen_height()) - ball_size) / 2.0, ball_direction, ball_size);
    
    (left_paddle, right_paddle, ball)
}

// To draw on the middle of the screen
fn draw_midle_text(txt: &str){
    let text_measurement  = measure_text(&txt, None, 48, 1.0);
    let x_position = (screen_width() - text_measurement .width) / 2.0;
    let y_position = (screen_height() - text_measurement .height) / 2.0;
    draw_text(&txt, x_position, y_position, 48.0, WHITE);
}

#[macroquad::main(window_config)]
async fn main() {
    let paddle_size = vec2(20.0, 60.0);
    let paddle_speed = 15.0; 

    let ball_size = vec2(20.0, 20.0);
    let mut ball_speed = 5.0; // mut bcs it encrease in the time
    
    let (mut left_paddle, mut right_paddle, mut ball) = reset_game(paddle_size, ball_size);

    let mut left_score = 0;
    let mut right_score = 0;
    
    let text_versus = measure_text(&"VS", None, 48, 1.0);
    
    /* Little lapse screen */
    let mut middle_text = "START";
    let mut delay_frames = 30;

    loop {
        if delay_frames > 0
        {
            delay_frames -= 1;
            draw_midle_text(middle_text);
            next_frame().await;
            continue;
        }        
        //Clear screen
        clear_background(BLACK);
        
        let text_measurement = measure_text(&left_score.to_string(), None, 48, 1.0);
        draw_text(&left_score.to_string(), (screen_width() - text_measurement.width) / 2.0 - 100.0, 50.0, 48.0, WHITE);
        let text_measurement = measure_text(&right_score.to_string(), None, 48, 1.0);
        draw_text(&right_score.to_string(), (screen_width() - text_measurement.width) / 2.0 + 100.0, 50.0, 48.0, WHITE);
        draw_text(&"VS", (screen_width() - text_versus.width) / 2.0, 50.0, 48.0, WHITE);
        
        left_paddle.update(paddle_speed, KeyCode::W, KeyCode::S);
        left_paddle.draw();
        right_paddle.update(paddle_speed, KeyCode::Up, KeyCode::Down);
        right_paddle.draw();

        if ball_speed < 15.0{
            ball_speed += 0.5;
        }
        ball.update(ball_speed / 2.0, &left_paddle, &right_paddle);
        ball.draw();

        if ball.position.x <= 0.0 {
            right_score += 1;
            (left_paddle, right_paddle, ball) = reset_game(paddle_size, ball_size);
            middle_text = "Left win";
            delay_frames = 60
        }
        if ball.position.x >= screen_width() - ball.size.x {
            left_score += 1;
            (left_paddle, right_paddle, ball) = reset_game(paddle_size, ball_size);
            middle_text = "Right win";
            delay_frames = 60
        }

        next_frame().await;
    }
}
