use ggez::nalgebra::Point2;

pub struct Ball {
    pub position: Point2<f32>,
    direction: Point2<f32>,
    velocity: f32,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            position: Point2::new(200.0, 200.0),
            direction: Point2::new(1.0, 1.0),
            velocity: 3.0,
        }
    }

    pub fn update(&mut self, ball_touches_paddle: bool) {
        if ball_touches_paddle {
            self.direction.x *= -1.0;
            self.direction.y *= -1.0;
        } else {
            if self.position.x > 800.0 || self.position.x < 0.0 {
                self.direction.x *= -1.0;
            }

            if self.position.y > 600.0 || self.position.y < 0.0 {
                self.direction.y *= -1.0;
            }
        }

        self.position.x += self.direction.x * self.velocity;
        self.position.y += self.direction.y * self.velocity;
    }
}
