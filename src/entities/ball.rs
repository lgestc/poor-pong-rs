use ggez::nalgebra::Point2;

pub struct Ball {
    pub position: Point2<f32>,
    direction: Point2<f32>,
    base_velocity: f32,
    velocity: f32,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            position: Point2::new(200.0, 200.0),
            direction: Point2::new(1.0, 1.0),
            base_velocity: 3.0,
            velocity: 3.0,
        }
    }

    pub fn update<'a>(
        &mut self,
        paddle_bounds: ggez::graphics::Rect,
        on_outside_screen: Box<dyn FnOnce() + 'a>,
    ) {
        let mut ball_touches_paddle = false;

        if self.position.x <= paddle_bounds.w {
            if self.position.y >= paddle_bounds.y
                && self.position.y <= paddle_bounds.y + paddle_bounds.h
            {
                ball_touches_paddle = true;
            }
        }

        if ball_touches_paddle {
            let offset_center: f32 = -0.5;
            let mut contact_position = (self.position.y - paddle_bounds.y).abs();
            contact_position = contact_position / paddle_bounds.h; // in range of 0-1.0f

            self.direction.x *= -1.0;
            self.direction.y = (offset_center + contact_position) * (1.0 + contact_position.abs());

            self.velocity = self.base_velocity * (1.0 + (offset_center + contact_position).abs());
        } else {
            if self.position.x >= 800.0 || self.position.x <= 0.0 {
                on_outside_screen();
            }

            if self.position.y >= 600.0 || self.position.y <= 0.0 {
                self.direction.y *= -1.0;
            }
        }

        self.position.x += self.direction.x * self.velocity;
        self.position.y += self.direction.y * self.velocity;
    }
}
