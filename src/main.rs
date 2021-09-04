//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

mod distance_between;

use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use glam::*;
use ggez::input::mouse::button_pressed;
use ggez::input::mouse;
use rand;

use distance_between::distance_between;

struct Target {
    x: f32,
    y: f32,
    radius: f32,

    weight: f32,
    acc: f32,
    color: Color,
}

struct MainState {
    clicking: bool,
    target: Target,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState{
            clicking: false,
            target: Target {
                x: 100.,
                y: 100.,
                radius: 40.0,
                weight: 0.003,
                acc: 0.0,
                color: Color::WHITE,
            }
        };
        Ok(s)
    }

    fn reset_target(&mut self) {
        self.target.y = 100.0;
        self.target.acc = 0.0;
        self.target.weight += 0.0001;
        
        let width: f32 = rand::random();
        self.target.x = width * 800.;
    }

    fn reset_game(&mut self) {
        self.target.weight = 0.0;
        self.reset_target();
    }
}


impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.target.acc += self.target.weight * ggez::timer::delta(&_ctx).as_millis() as f32;
        self.target.y += self.target.acc;

        if button_pressed(&_ctx, mouse::MouseButton::Left,) {

            if !self.clicking {
                let mouse_pos = mouse::position(&_ctx);
                // Calculate distance between the inner circle and the mouse to see if the mouse is in the circle
                if distance_between(mouse_pos.x, mouse_pos.y, self.target.x, self.target.y) < self.target.radius {
                    // if the circle is clicked
                    self.reset_target();
                }
                self.clicking = true;
            }
            
            
        } else { self.clicking = false; }


        if self.target.y + self.target.radius > 600. {
            self.reset_game();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let target = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            self.target.radius,
            0.1,
            self.target.color,
        )?;
        graphics::draw(ctx, &target, (Vec2::new(self.target.x, self.target.y),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;

    // let mode = ggez::conf::WindowMode{
    //     width: 800.0,
    //     height: 600.,
    //     borderless: false,
    //     fullscreen_type: ggez::conf::FullscreenType::Windowed,
    //     min_width: 0.,
    //     max_width: 0.,
    //     min_height: 0.,
    //     max_height: 0.,
    //     maximized: false,
    //     resizable: false,
    //     visible: false,
    //     resize_on_scale_factor_change: false,

    // };
    //let _res = graphics::set_mode(&mut ctx, mode);

    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}