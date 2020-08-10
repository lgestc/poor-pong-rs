use ggez::nalgebra::Point2;

pub struct Ball {
    pub position: Point2<f32>,
    pub direction: Point2<f32>,
    pub base_velocity: f32,
    pub velocity: f32,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            position: Point2::new(600.0, 300.0),
            direction: Point2::new(-1.0, 0.0),
            base_velocity: 4.0,
            velocity: 4.0,
        }
    }

    pub fn update_position(&mut self) {
        self.position.x += self.direction.x * self.velocity;
        self.position.y += self.direction.y * self.velocity;
    }
}
