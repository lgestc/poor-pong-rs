use crate::entities::{ball::Ball, paddle::MovementDirection, paddle::Paddle};

pub struct World {
    pub ball: Ball,
    pub paddle: Paddle,
    pub paddle2: Paddle,
    pub movement: MovementDirection,
    pub paddle_height: f32,
    pub paddle_width: f32,
    target_opponent_y: f32,
}

impl World {
    fn reset() -> Self {
        Self {
            ball: Ball::new(),
            paddle: Paddle::new(),
            paddle2: Paddle::new(),
            movement: MovementDirection::None,
            paddle_height: 128.0,
            paddle_width: 16.0,
            target_opponent_y: 0.0,
        }
    }

    pub fn new() -> Self {
        Self::reset()
    }

    pub fn update(&mut self) {
        self.paddle.update_position(&self.movement);
        self.paddle2.position.x = 800.0 - self.paddle_width;

        if (self.paddle2.position.y - self.target_opponent_y).abs() > 16.0
            && self.ball.direction.x > 0.0
        {
            if self.paddle2.position.y < self.target_opponent_y {
                self.paddle2.update_position(&MovementDirection::Down);
            } else if self.paddle2.position.y > self.target_opponent_y {
                self.paddle2.update_position(&MovementDirection::Up);
            }
        }

        if self.ball.position.y >= 600.0 || self.ball.position.y <= 0.0 {
            self.ball.direction.y *= -1.0;
        }

        if self.ball.direction.x > 0.0
            && self.ball.position.x >= 450.0
            && self.ball.position.x <= 716.0
        {
            self.target_opponent_y = self.ball.position.y - self.paddle_height / 2.0;
        }

        self.check_ball_collision();
        self.ball.update_position();

        if self.ball.position.x > 800.0 || self.ball.position.x < 0.0 {
            let reset_world = Self::reset();

            self.ball = reset_world.ball;
            self.paddle = reset_world.paddle;
            self.paddle2 = reset_world.paddle2;
            self.target_opponent_y = reset_world.target_opponent_y;
        }
    }

    fn check_ball_collision(&mut self) {
        let paddle_bounds_vec = vec![
            ggez::graphics::Rect::new(
                self.paddle.position.x,
                self.paddle.position.y,
                self.paddle_width,
                self.paddle_height,
            ),
            ggez::graphics::Rect::new(
                self.paddle2.position.x,
                self.paddle2.position.y,
                self.paddle_width,
                self.paddle_height,
            ),
        ];

        for paddle_bounds in paddle_bounds_vec.iter() {
            if self.ball.position.x <= paddle_bounds.x + paddle_bounds.w
                && self.ball.position.x >= paddle_bounds.x
                && self.ball.position.y >= paddle_bounds.y
                && self.ball.position.y <= paddle_bounds.y + paddle_bounds.h
            {
                let offset_center: f32 = -0.5;
                let mut contact_position = (self.ball.position.y - paddle_bounds.y).abs();
                contact_position = contact_position / paddle_bounds.h; // in range of 0-1.0f

                self.ball.direction.x *= -1.0;

                self.ball.direction.y =
                    (offset_center + contact_position) * (1.0 + contact_position.abs());

                if self.ball.position.x > 400.0 {
                    self.ball.position.x -= 8.0;
                }

                self.ball.velocity =
                    self.ball.base_velocity * (1.0 + (offset_center + contact_position).abs());

                self.ball.position.x += self.ball.direction.x * self.ball.velocity;
                self.ball.position.y += self.ball.direction.y * self.ball.velocity;
            }
        }
    }
}
