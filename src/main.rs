extern crate ggez;

pub mod world;
pub mod camera;
pub mod assets;

use ggez::conf::Conf;
use ggez::event;
use ggez::{GameResult, Context};
use ggez::graphics;
use std::time::Duration;
use world::World;
use camera::Camera;

const VIEWPORT_HEIGHT: u32 = 400;
const VIEWPORT_WIDTH: u32 = 400;

struct MainState {
    world: World,
    camera: Camera,
}

impl MainState {
    fn new(world: World, camera: Camera) -> GameResult<MainState> {
        Ok(MainState {
            world,
            camera,
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
        let x_offset = self.camera.get_left() % world::TILE_WIDTH as f32;
        let y_offset = self.camera.get_top() % world::TILE_HEIGHT as f32;

        for (i, row) in tiles.iter().enumerate() {
            let i = i as f32;
            for (j, tile) in row.iter().enumerate() {
                let j = j as f32;

                graphics::draw(ctx, &*tile.meta.image, graphics::Point::new(j * world::TILE_WIDTH as f32 + world::TILE_WIDTH as f32 / 2.0 - x_offset, i * world::TILE_HEIGHT as f32 + world::TILE_HEIGHT as f32 / 2.0 - y_offset), 0.0)?;
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
    let c = Conf {
        window_title: String::from("World's best game"),
        window_icon: String::from(""),
        window_height: VIEWPORT_HEIGHT,
        window_width: VIEWPORT_WIDTH,
        vsync: false,
        resizable: false
    };
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();

    let mut world = world::World::new(String::from("New game"), ctx);
    world.load_world_1();

    let cam = camera::Camera::new(VIEWPORT_HEIGHT, VIEWPORT_WIDTH, world.get_width(), world.get_height());
    let state = &mut MainState::new(world, cam).unwrap();
    event::run(ctx, state).unwrap();
}
