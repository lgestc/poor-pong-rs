use ggez::nalgebra::Point2;

pub enum MovementDirection {
    Up,
    Down,
    None,
}
pub struct Paddle {
    pub position: Point2<f32>,
    direction: Point2<f32>,
    velocity: f32,
}

impl Paddle {
    pub fn new() -> Self {
        Paddle {
            position: Point2::new(0.0, 0.0),
            direction: Point2::new(0.0, 0.0),
            velocity: 6.0,
        }
    }

    pub fn update(&mut self, movement_direction: &MovementDirection) {
        match movement_direction {
            MovementDirection::Down => {
                self.direction.y = 1.0;
            }

            MovementDirection::Up => {
                self.direction.y = -1.0;
            }

            _ => {
                self.direction.y = 0.0;
            }
        }

        self.position.x += self.direction.x * self.velocity;
        self.position.y += self.direction.y * self.velocity;
    }
}
