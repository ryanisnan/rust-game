extern crate ggez;

pub mod world;
pub mod camera;

use ggez::conf;
use ggez::event;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, Point};
use std::time::Duration;


struct MainState {
    world: world::World,
    camera: camera::Camera
}

impl MainState {
    fn new(ctx: &mut Context, world: world::World, camera: camera::Camera) -> GameResult<MainState> {
        let s = MainState { world, camera };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        let tiles = self.world.get_tiles(self.camera.get_left(), self.camera.get_right(), self.camera.get_top(), self.camera.get_bottom());
        let x_offset = self.camera.get_left() % self.world.tile_width;
        let y_offset = self.camera.get_top() % self.world.tile_height;

        for (i, row) in tiles.iter().enumerate() {
            let i = i as f32;
            for (j, tile) in row.iter().enumerate() {
                let j = j as f32;

                graphics::set_color(ctx, Color::from(tile.get_color()))?;
                graphics::rectangle(
                    ctx,
                    DrawMode::Fill,
                    graphics::Rect {
                        x: j * self.world.tile_width + self.world.tile_width / 2.0 - x_offset,
                        y: i * self.world.tile_height + self.world.tile_height / 2.0 - y_offset,
                        w: self.world.tile_width,
                        h: self.world.tile_height
                    }
                )?;
            }
        }
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, keycode: event::Keycode, keymod: event::Mod, repeat: bool) {
        match keycode {
            event::Keycode::Left => {
                self.camera.move_left();
            },
            event::Keycode::Right => {
                self.camera.move_right();
            },
            event::Keycode::Down => {
                self.camera.move_down();
            },
            event::Keycode::Up => {
                self.camera.move_up();
            },
            _ => {
            }
        }
        println!(
            "Key pressed: {:?}, modifier {:?}, repeat: {}",
            keycode,
            keymod,
            repeat
        );
    }
}


fn main() {
    let mut world = world::World::generate_world_1();
    let mut cam = camera::Camera::new(800.0, 800.0, world.get_width(), world.get_height());

    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx, world, cam).unwrap();
    event::run(ctx, state).unwrap();
    //
    // while let Some(event) = window.next() {
    //     match event.press_args() {
    //         Some(piston_window::Button::Keyboard(piston_window::Key::Left)) => {
    //
    //         },
    //         Some(piston_window::Button::Keyboard(piston_window::Key::Right)) => {
    //             cam.move_right();
    //         },
    //         Some(piston_window::Button::Keyboard(piston_window::Key::Up)) => {
    //             cam.move_up();
    //         },
    //         Some(piston_window::Button::Keyboard(piston_window::Key::Down)) => {
    //             cam.move_down();
    //         },
    //         Some(_) => (),
    //         None => ()
    //     }
    //
    //
    // }
}
