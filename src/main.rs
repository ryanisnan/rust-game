extern crate ggez;

pub mod world;
pub mod camera;
pub mod assets;

use ggez::conf;
use ggez::event;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::graphics::{Image, DrawMode, Point};
use std::time::Duration;
use assets::AssetLoader;
use world::World;
use camera::Camera;

struct MainState {
    world: World,
    camera: Camera,
    asset_loader: AssetLoader,
}

impl MainState {
    fn new(world: World, camera: Camera, asset_loader: AssetLoader) -> GameResult<MainState> {
        Ok(MainState {
            world,
            camera,
            asset_loader
        })
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

                let img = tile.get_image(ctx, &mut self.asset_loader);
                graphics::draw(ctx, img, graphics::Point::new(j * self.world.tile_width + self.world.tile_width / 2.0 - x_offset, i * self.world.tile_height + self.world.tile_height / 2.0 - y_offset), 0.0)?;
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
    let mut asset_loader = assets::AssetLoader::new();

    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(world, cam, asset_loader).unwrap();

    event::run(ctx, state).unwrap();

}
