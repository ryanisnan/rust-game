extern crate ggez;

pub mod world;
pub mod camera;
pub mod assets;

use ggez::conf;
use ggez::event;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, Point};
use std::time::Duration;
use assets::Loadable;


struct MainState {
    world: world::World,
    camera: camera::Camera,
    image: graphics::Image,
}

impl MainState {
    fn new(ctx: &mut Context, world: world::World, camera: camera::Camera, image: graphics::Image) -> GameResult<MainState> {
        let s = MainState { world, camera, image };
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

                graphics::draw(ctx, &self.image, graphics::Point::new(j * self.world.tile_width + self.world.tile_width / 2.0 - x_offset, i * self.world.tile_height + self.world.tile_height / 2.0 - y_offset), 0.0)?;
                // graphics::set_color(ctx, Color::from(tile.get_color()))?;
                // graphics::rectangle(
                //     ctx,
                //     DrawMode::Fill,
                //     graphics::Rect {
                //         x: j * self.world.tile_width + self.world.tile_width / 2.0 - x_offset,
                //         y: i * self.world.tile_height + self.world.tile_height / 2.0 - y_offset,
                //         w: self.world.tile_width,
                //         h: self.world.tile_height
                //     }
                // )?;
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

    // let mut loader = AssetLoader::new();

    // let asset1 = loader.load::<AssetType>(&mut ctx, "dragon1.png");

    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let image = graphics::Image::new(ctx, "/grass-1.png").unwrap();
    let state = &mut MainState::new(ctx, world, cam, image).unwrap();

    let asset_loader = assets::AssetLoader::new();
    asset_loader.load::<ggez::graphics::Image>(ctx, "/grass-1.png");
    // let asset1 = ggez::graphics::Image::load(ctx, "/grass-1.png");
    // println!("{:?}", asset1);


    event::run(ctx, state).unwrap();

}
