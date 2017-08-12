extern crate ggez;
extern crate rusty_engine;

use ggez::{GameResult, Context};
use ggez::conf::Conf;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{Image, Rect};
use std::time::Duration;

use rusty_engine::camera;
use rusty_engine::camera::Camera;
use rusty_engine::decoration::{DecorationPrototype, DecorationLibrary, Decoration};
use rusty_engine::tile::{TilePrototype, TileLibrary, Tile};
use rusty_engine::world;
use rusty_engine::world::World;
use rusty_engine::renderer::Renderer;

const VIEWPORT_HEIGHT: u32 = 300;
const VIEWPORT_WIDTH: u32 = 300;

fn load_image(ctx: &mut Context, file_path: &str) -> Image {
     Image::new(ctx, file_path).unwrap()
}

fn populate_tile_library(tile_lib: &mut TileLibrary, ctx: &mut Context) {
    tile_lib.load("GrassLight", TilePrototype { image: load_image(ctx, "/lgrass.png"), is_walkable: true });
    tile_lib.load("GrassLight2", TilePrototype { image: load_image(ctx, "/lgrass2.png"), is_walkable: true });
    tile_lib.load("GrassLight3", TilePrototype { image: load_image(ctx, "/lgrass3.png"), is_walkable: true });
    tile_lib.load("GrassDark", TilePrototype { image: load_image(ctx, "/dgrass.png"), is_walkable: true });
    tile_lib.load("Sand", TilePrototype { image: load_image(ctx, "/sand.png"), is_walkable: true });
    tile_lib.load("Sand2", TilePrototype { image: load_image(ctx, "/sand2.png"), is_walkable: true });
    tile_lib.load("Sand3", TilePrototype { image: load_image(ctx, "/sand3.png"), is_walkable: true });
    tile_lib.load("GrassSandNW", TilePrototype { image: load_image(ctx, "/grass-sand-nw.png"), is_walkable: true });
    tile_lib.load("GrassSandN", TilePrototype { image: load_image(ctx, "/grass-sand-n.png"), is_walkable: true });
    tile_lib.load("GrassSandN2", TilePrototype { image: load_image(ctx, "/grass-sand-n2.png"), is_walkable: true });
    tile_lib.load("GrassSandN3", TilePrototype { image: load_image(ctx, "/grass-sand-n3.png"), is_walkable: true });
    tile_lib.load("GrassSandNE", TilePrototype { image: load_image(ctx, "/grass-sand-ne.png"), is_walkable: true });
    tile_lib.load("GrassSandE", TilePrototype { image: load_image(ctx, "/grass-sand-e.png"), is_walkable: true });
    tile_lib.load("GrassSandE2", TilePrototype { image: load_image(ctx, "/grass-sand-e2.png"), is_walkable: true });
    tile_lib.load("GrassSandE3", TilePrototype { image: load_image(ctx, "/grass-sand-e3.png"), is_walkable: true });
    tile_lib.load("GrassSandSE", TilePrototype { image: load_image(ctx, "/grass-sand-se.png"), is_walkable: true });
    tile_lib.load("GrassSandS", TilePrototype { image: load_image(ctx, "/grass-sand-s.png"), is_walkable: true });
    tile_lib.load("GrassSandS2", TilePrototype { image: load_image(ctx, "/grass-sand-s2.png"), is_walkable: true });
    tile_lib.load("GrassSandS3", TilePrototype { image: load_image(ctx, "/grass-sand-s3.png"), is_walkable: true });
    tile_lib.load("GrassSandSW", TilePrototype { image: load_image(ctx, "/grass-sand-sw.png"), is_walkable: true });
    tile_lib.load("GrassSandW", TilePrototype { image: load_image(ctx, "/grass-sand-w.png"), is_walkable: true });
    tile_lib.load("GrassSandW2", TilePrototype { image: load_image(ctx, "/grass-sand-w2.png"), is_walkable: true });
    tile_lib.load("GrassSandW3", TilePrototype { image: load_image(ctx, "/grass-sand-w3.png"), is_walkable: true });
    tile_lib.load("SandGrassNW", TilePrototype { image: load_image(ctx, "/sand-grass-nw.png"), is_walkable: true });
    tile_lib.load("SandGrassNE", TilePrototype { image: load_image(ctx, "/sand-grass-ne.png"), is_walkable: true });
    tile_lib.load("SandGrassSE", TilePrototype { image: load_image(ctx, "/sand-grass-se.png"), is_walkable: true });
    tile_lib.load("SandGrassSW", TilePrototype { image: load_image(ctx, "/sand-grass-sw.png"), is_walkable: true });
}

fn populate_decoration_library(decoration_lib: &mut DecorationLibrary, ctx: &mut Context) {
    decoration_lib.load("Bush", DecorationPrototype { image: load_image(ctx, "/bush1.png") });
    decoration_lib.load("Stone", DecorationPrototype { image: load_image(ctx, "/stone.png") });
    decoration_lib.load("Stones", DecorationPrototype { image: load_image(ctx, "/stones.png") });
}

struct MainState {
    renderer: Renderer,
    tile_lib: TileLibrary,
    decoration_lib: DecorationLibrary,
    world: World,
    camera: Camera,
    changed: bool
}

impl MainState {
    fn new(world: World, camera: Camera) -> MainState {
        MainState {
            renderer: Renderer::new(),
            tile_lib: TileLibrary::new(),
            decoration_lib: DecorationLibrary::new(),
            world,
            camera,
            changed: true,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.changed {
            graphics::clear(ctx);
            self.world.show_indexes(&self.camera);
            let tiles = self.world.get_visible_subset(&self.camera);

            let mut x_offset = 0.0;
            if (self.camera.left() % world::TILE_WIDTH as f32).abs() != 0.0 {
                x_offset = world::TILE_WIDTH as f32 - (self.camera.left() % world::TILE_WIDTH as f32).abs();
            }
            // let x_offset = 128.0 - ().abs();
            let y_offset = self.camera.top().abs() % world::TILE_HEIGHT as f32;

            for (i, row) in tiles.iter().enumerate() {
                let i = i as f32;
                for (j, tile) in row.iter().enumerate() {
                    let j = j as f32;

                    let t_x: f32 = j * world::TILE_WIDTH as f32 + world::TILE_WIDTH as f32 / 2.0 - x_offset;
                    let t_y: f32 = i * world::TILE_HEIGHT as f32 + world::TILE_HEIGHT as f32 / 2.0;
                    let mut p = graphics::Point::new(t_x, t_y);

                    if i == 0.0 {
                        println!("tx: {}, ty: {} (Offset: {})", t_x, t_y, x_offset);
                    }
                    graphics::draw(ctx, &tile.meta.image, p, 0.0)?;

                    for decoration in tile.decorations.iter() {
                        graphics::draw(ctx, &decoration[0].meta.image, p, 0.0)?;
                    }
                }
            }

            self.changed = false;
        }
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, keycode: event::Keycode, keymod: event::Mod, repeat: bool) {
        self.changed = true;
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

        // self.world.show_indexes(&self.camera);
        // println!(
            // "Key pressed: {:?}, modifier {:?}, repeat: {}",
            // keycode,
            // keymod,
            // repeat
        // );
    }
}

fn load_world_1(world: &mut World, tile_lib: &TileLibrary, decoration_lib: &DecorationLibrary) {
    // Row 1
    let r1 = vec![
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
    ];
    world.data.push(r1);

    // Row 2
    let r2 = vec![
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: Some(vec![Decoration{meta: decoration_lib.decorations["Bush"].clone()}])},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
    ];
    world.data.push(r2);

    // Row 3
    let r3 = vec![
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassSandNW"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassSandN"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassSandN3"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassSandNE"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
    ];
    world.data.push(r3);

    // Row 4
    let r4 = vec![
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassSandSW"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["SandGrassNE"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["Sand"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassSandE3"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
    ];
    world.data.push(r4);

    // Row 5
    let r5 = vec![
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassSandSW"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassSandS"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassSandSE"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
    ];
    world.data.push(r5);

    // Row 6
    let r6 = vec![
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: Some(vec![Decoration{meta: decoration_lib.decorations["Stones"].clone()}])},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
    ];
    world.data.push(r6);

    // Row 7
    let r7 = vec![
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: Some(vec![Decoration{meta: decoration_lib.decorations["Stones"].clone()}])},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: Some(vec![Decoration{meta: decoration_lib.decorations["Stones"].clone()}])},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: Some(vec![Decoration{meta: decoration_lib.decorations["Stones"].clone()}])},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
    ];
    world.data.push(r7);

    let r8 = vec![
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
        Tile { meta: tile_lib.tiles["GrassLight"].clone(), decorations: None},
    ];
    world.data.push(r8);
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
    let mut tile_lib = TileLibrary::new();
    let mut decoration_lib = DecorationLibrary::new();
    populate_tile_library(&mut tile_lib, ctx);
    populate_decoration_library(&mut decoration_lib, ctx);

    load_world_1(&mut world, &tile_lib, &decoration_lib);

    let cam = camera::Camera::new(Rect {x: 0.0, y: 0.0, w: VIEWPORT_WIDTH as f32, h: VIEWPORT_HEIGHT as f32}, Rect{x: 0.0, y: 0.0, w: world.width() as f32, h: world.height() as f32});
    let mut state = MainState::new(world, cam);
    event::run(ctx,&mut state).unwrap();
}
