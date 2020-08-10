use ggez::nalgebra::Point2;

pub struct Ball {
    pub position: Point2<f32>,
    direction: Point2<f32>,
    base_velocity: f32,
    velocity: f32,
}

pub enum UpdateResult {
    Continue,
    OutsideScreen,
    StartMovement,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            position: Point2::new(400.0, 500.0),
            direction: Point2::new(-1.0, -1.0),
            base_velocity: 6.0,
            velocity: 6.0,
        }
    }

    pub fn update<'a>(&mut self, paddle_bounds_vec: Vec<ggez::graphics::Rect>) -> UpdateResult {
        for paddle_bounds in paddle_bounds_vec.iter() {
            if self.position.x <= paddle_bounds.x + paddle_bounds.w
                && self.position.x >= paddle_bounds.x
                && self.position.y >= paddle_bounds.y
                && self.position.y <= paddle_bounds.y + paddle_bounds.h
            {
                let offset_center: f32 = -0.5;
                let mut contact_position = (self.position.y - paddle_bounds.y).abs();
                contact_position = contact_position / paddle_bounds.h; // in range of 0-1.0f

                self.direction.x *= -1.0;
                self.direction.y =
                    (offset_center + contact_position) * (1.0 + contact_position.abs());

                if self.position.x > 400.0 {
                    self.position.x -= 8.0;
                }

                self.velocity =
                    self.base_velocity * (1.0 + (offset_center + contact_position).abs());

                self.position.x += self.direction.x * self.velocity;
                self.position.y += self.direction.y * self.velocity;

                return UpdateResult::Continue;
            }
        }

        if self.position.x >= 800.0 || self.position.x <= 0.0 {
            return UpdateResult::OutsideScreen;
        }

        if self.position.y >= 600.0 || self.position.y <= 0.0 {
            self.direction.y *= -1.0;
        }

        self.position.x += self.direction.x * self.velocity;
        self.position.y += self.direction.y * self.velocity;

        if self.position.x >= 500.0 && self.position.x <= 532.0 {
            return UpdateResult::StartMovement;
        }

        UpdateResult::Continue
    }
}
