use ggez::{
    event::{EventHandler, KeyCode, KeyMods},
    graphics, nalgebra as na, timer, Context, GameResult,
};

use std::time::{Duration, Instant};

use crate::{entities::paddle::MovementDirection, world::World};

pub struct MyGame {
    world: World,
    accumulated_time: f32,
    fps_last_update: Instant,
    fps_readings: Vec<f32>,
    last_update: Instant,
    last_draw: Instant,
    fixed_time_step: f32,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let updates_per_second = 100;

        MyGame {
            world: World::new(),
            last_update: Instant::now(),
            last_draw: Instant::now(),
            accumulated_time: 0.0,
            fps_last_update: Instant::now(),
            fps_readings: vec![],
            fixed_time_step: 1.0 / updates_per_second as f32,
        }
    }

    pub fn restart(&mut self) {
        self.world = World::new();
    }
}

impl EventHandler for MyGame {
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Up => {
                self.world.movement = MovementDirection::Up;
            }
            KeyCode::Down => {
                self.world.movement = MovementDirection::Down;
            }
            _ => {}
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {
        self.world.movement = MovementDirection::None;
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let diff: Duration = Instant::now() - self.last_update;
        let delta = diff.as_secs_f32();
        self.accumulated_time += delta;

        while self.accumulated_time >= self.fixed_time_step {
            self.world.update();
            self.accumulated_time -= self.fixed_time_step;
        }

        self.last_update = Instant::now();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let fps: Duration = Instant::now() - self.last_draw;
        let fps = 1.0 / fps.as_secs_f32();

        self.fps_readings.push(fps);

        let fps_time_since_update: Duration = Instant::now() - self.fps_last_update;

        if fps_time_since_update.as_secs() >= 5 {
            let mut avg_fps = 0.0;
            for reading in self.fps_readings.iter() {
                avg_fps += reading;
            }

            println!("avg fps (5s): {}", avg_fps / self.fps_readings.len() as f32);

            self.fps_last_update = Instant::now();
            self.fps_readings.clear();
        }

        let ball_sprite = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            self.world.ball.position,
            3.0,
            1.0,
            graphics::WHITE,
        )?;

        let paddle_sprite = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            ggez::graphics::Rect::new(
                self.world.paddle.position.x,
                self.world.paddle.position.y,
                self.world.paddle_width,
                self.world.paddle_height,
            ),
            graphics::WHITE,
        )?;

        let paddle_sprite2 = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            ggez::graphics::Rect::new(
                self.world.paddle2.position.x,
                self.world.paddle2.position.y,
                self.world.paddle_width,
                self.world.paddle_height,
            ),
            graphics::Color::from_rgb(255, 0, 0),
        )?;

        graphics::draw(ctx, &ball_sprite, (na::Point2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &paddle_sprite, (na::Point2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &paddle_sprite2, (na::Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)?;

        timer::yield_now();

        self.last_draw = Instant::now();

        Ok(())
    }
}
