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

                graphics::set_color(ctx, graphics::Color::from(tile.get_color()));
                graphics::rectangle(
                    ctx,
                    DrawMode::Fill,
                    graphics::Rect {
                        x: j * self.world.tile_width - x_offset,
                        y: i * self.world.tile_height - y_offset,
                        w: self.world.tile_width,
                        h: self.world.tile_height
                    }
                )?;
            }
        }
        // graphics::circle(
        //     ctx,
        //     DrawMode::Fill,
        //     Point {
        //         x: self.pos_x,
        //         y: 380.0,
        //     },
        //     100.0,
        //     32,
        // )?;
        graphics::present(ctx);
        Ok(())
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
    //             cam.move_left();
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
    //     let tiles = world.get_tiles(cam.get_left(), cam.get_right(), cam.get_top(), cam.get_bottom());
    //     let x_offset = cam.get_left() % world.tile_width;
    //     let y_offset = cam.get_top() % world.tile_height;
    //
    //     // println!("{:#?}", event);
    //     // This runs many times per second...
    //     window.draw_2d(&event, |context, graphics| {
    //
    //         piston_window::clear([1.0; 4], graphics);
    //
    //         for (i, row) in tiles.iter().enumerate() {
    //             let i = i as f64;
    //             for (j, tile) in row.iter().enumerate() {
    //                 let j = j as f64;
    //                 piston_window::rectangle(
    //                     tile.get_color(),
    //                     [j * world.tile_width - x_offset, i * world.tile_height - y_offset, world.tile_height, world.tile_width],
    //                     context.transform,
    //                     graphics
    //                 );
    //             }
    //         }
    //     });
    // }
}
