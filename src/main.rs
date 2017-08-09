extern crate ggez;
extern crate rusty_engine;

use ggez::conf::Conf;
use ggez::event;
use ggez::{GameResult, Context};
use ggez::graphics;
use std::time::Duration;
use rusty_engine::world;
use rusty_engine::camera;
use rusty_engine::world::{World, TileType, TileLibrary, Tile};
use rusty_engine::entity::{DecorationType, DecorationLibrary, Decoration};
use rusty_engine::camera::Camera;
use rusty_engine::assets::AssetLoader;

const VIEWPORT_HEIGHT: u32 = 768;
const VIEWPORT_WIDTH: u32 = 1024;


fn populate_tile_library(tile_lib: &mut TileLibrary, asset_loader: &mut AssetLoader, ctx: &mut Context) {
    tile_lib.load("GrassLight", TileType { image: asset_loader.load_image(ctx, "/lgrass.png"), is_walkable: true });
    tile_lib.load("GrassLight2", TileType { image: asset_loader.load_image(ctx, "/lgrass2.png"), is_walkable: true });
    tile_lib.load("GrassLight3", TileType { image: asset_loader.load_image(ctx, "/lgrass3.png"), is_walkable: true });
    tile_lib.load("GrassDark", TileType { image: asset_loader.load_image(ctx, "/dgrass.png"), is_walkable: true });
    tile_lib.load("Sand", TileType { image: asset_loader.load_image(ctx, "/sand.png"), is_walkable: true });
    tile_lib.load("Sand2", TileType { image: asset_loader.load_image(ctx, "/sand2.png"), is_walkable: true });
    tile_lib.load("Sand3", TileType { image: asset_loader.load_image(ctx, "/sand3.png"), is_walkable: true });
    tile_lib.load("GrassSandNW", TileType { image: asset_loader.load_image(ctx, "/grass-sand-nw.png"), is_walkable: true });
    tile_lib.load("GrassSandN", TileType { image: asset_loader.load_image(ctx, "/grass-sand-n.png"), is_walkable: true });
    tile_lib.load("GrassSandN2", TileType { image: asset_loader.load_image(ctx, "/grass-sand-n2.png"), is_walkable: true });
    tile_lib.load("GrassSandN3", TileType { image: asset_loader.load_image(ctx, "/grass-sand-n3.png"), is_walkable: true });
    tile_lib.load("GrassSandNE", TileType { image: asset_loader.load_image(ctx, "/grass-sand-ne.png"), is_walkable: true });
    tile_lib.load("GrassSandE", TileType { image: asset_loader.load_image(ctx, "/grass-sand-e.png"), is_walkable: true });
    tile_lib.load("GrassSandE2", TileType { image: asset_loader.load_image(ctx, "/grass-sand-e2.png"), is_walkable: true });
    tile_lib.load("GrassSandE3", TileType { image: asset_loader.load_image(ctx, "/grass-sand-e3.png"), is_walkable: true });
    tile_lib.load("GrassSandSE", TileType { image: asset_loader.load_image(ctx, "/grass-sand-se.png"), is_walkable: true });
    tile_lib.load("GrassSandS", TileType { image: asset_loader.load_image(ctx, "/grass-sand-s.png"), is_walkable: true });
    tile_lib.load("GrassSandS2", TileType { image: asset_loader.load_image(ctx, "/grass-sand-s2.png"), is_walkable: true });
    tile_lib.load("GrassSandS3", TileType { image: asset_loader.load_image(ctx, "/grass-sand-s3.png"), is_walkable: true });
    tile_lib.load("GrassSandSW", TileType { image: asset_loader.load_image(ctx, "/grass-sand-sw.png"), is_walkable: true });
    tile_lib.load("GrassSandW", TileType { image: asset_loader.load_image(ctx, "/grass-sand-w.png"), is_walkable: true });
    tile_lib.load("GrassSandW2", TileType { image: asset_loader.load_image(ctx, "/grass-sand-w2.png"), is_walkable: true });
    tile_lib.load("GrassSandW3", TileType { image: asset_loader.load_image(ctx, "/grass-sand-w3.png"), is_walkable: true });
    tile_lib.load("SandGrassNW", TileType { image: asset_loader.load_image(ctx, "/sand-grass-nw.png"), is_walkable: true });
    tile_lib.load("SandGrassNE", TileType { image: asset_loader.load_image(ctx, "/sand-grass-ne.png"), is_walkable: true });
    tile_lib.load("SandGrassSE", TileType { image: asset_loader.load_image(ctx, "/sand-grass-se.png"), is_walkable: true });
    tile_lib.load("SandGrassSW", TileType { image: asset_loader.load_image(ctx, "/sand-grass-sw.png"), is_walkable: true });
}

fn populate_decoration_library(decoration_lib: &mut DecorationLibrary, asset_loader: &mut AssetLoader, ctx: &mut Context) {
    decoration_lib.load("Bush", DecorationType { image: asset_loader.load_image(ctx, "/bush1.png") });
    decoration_lib.load("Stone", DecorationType { image: asset_loader.load_image(ctx, "/stone.png") });
    decoration_lib.load("Stones", DecorationType { image: asset_loader.load_image(ctx, "/stones.png") });
}

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

                let p = graphics::Point::new(j * world::TILE_WIDTH as f32 + world::TILE_WIDTH as f32 / 2.0 - x_offset, i * world::TILE_HEIGHT as f32 + world::TILE_HEIGHT as f32 / 2.0 - y_offset);
                graphics::draw(ctx, &*tile.meta.image, p, 0.0)?;

                for decoration in tile.decorations.iter() {
                    graphics::draw(ctx, &*decoration[0].meta.image, p, 0.0);
                }

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

fn load_world_1(world: &mut World) {
    // Row 1
    let r1 = vec![
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
    ];
    world.data.push(r1);

    // Row 2
    let r2 = vec![
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: Some(vec![Decoration{meta: world.decorations_library.decorations["Bush"].clone()}])},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
    ];
    world.data.push(r2);

    // Row 3
    let r3 = vec![
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassSandNW"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassSandN"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassSandN3"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassSandNE"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
    ];
    world.data.push(r3);

    // Row 4
    let r4 = vec![
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassSandSW"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["SandGrassNE"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["Sand"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassSandE3"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
    ];
    world.data.push(r4);

    // Row 5
    let r5 = vec![
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassSandSW"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassSandS"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassSandSE"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
    ];
    world.data.push(r5);

    // Row 6
    let r6 = vec![
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: Some(vec![Decoration{meta: world.decorations_library.decorations["Stones"].clone()}])},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
    ];
    world.data.push(r6);

    // Row 7
    let r7 = vec![
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: Some(vec![Decoration{meta: world.decorations_library.decorations["Stones"].clone()}])},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: Some(vec![Decoration{meta: world.decorations_library.decorations["Stones"].clone()}])},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: Some(vec![Decoration{meta: world.decorations_library.decorations["Stones"].clone()}])},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: world.tile_library.tiles["GrassLight"].clone(), decorations: None},
    ];
    world.data.push(r7);
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

    let mut world = world::World::new(String::from("New game"));
    world.rows = 7;
    populate_tile_library(&mut world.tile_library, &mut world.asset_loader, ctx);
    populate_decoration_library(&mut world.decorations_library, &mut world.asset_loader, ctx);
    load_world_1(&mut world);

    let cam = camera::Camera::new(VIEWPORT_HEIGHT, VIEWPORT_WIDTH, world.get_width(), world.get_height());
    let state = &mut MainState::new(world, cam).unwrap();
    event::run(ctx, state).unwrap();
}
