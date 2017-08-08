extern crate rand;
extern crate ggez;

use assets::AssetLoader;
use ggez::graphics::Image;
use ggez::Context;
use entity::{DecorationLibrary, Decoration, DecorationType};

use std::collections::HashMap;
use std::rc::Rc;

pub const TILE_WIDTH: u32 = 64;
pub const TILE_HEIGHT: u32 = 64;


#[derive(Debug, PartialEq, Eq, Hash)]
enum TileType {
    // Basic enum listing the various types of tiles within the world
    Grass,
    Water
}

#[derive(Debug)]
pub struct TileMeta {
    // Defines data common across various types of tiles (Flyweight Pattern)
    pub image: Rc<Image>,
    pub is_walkable: bool
}

#[derive(Debug)]
pub struct Tile {
    // Represents a game tile in the world
    pub meta: Rc<TileMeta>,
    pub decorations: Option<Vec<Decoration>>,
}

#[derive(Debug)]
pub struct TileLibrary {
    // Represents a library of different tile types
    tiles: HashMap<TileType, Rc<TileMeta>>
}

impl TileLibrary {
    pub fn new(ctx: &mut Context, asset_loader: &mut AssetLoader) -> Self {
        let mut lib = TileLibrary {
            tiles: HashMap::new(),
        };

        lib.populate(ctx, asset_loader);

        lib
    }

    fn populate(&mut self, ctx: &mut Context, asset_loader: &mut AssetLoader) {
        self.tiles.insert(TileType::Grass, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/grass-1.png"), is_walkable: true }));
        self.tiles.insert(TileType::Water, Rc::new(TileMeta { image: asset_loader.load_image(ctx, "/water-3.png"), is_walkable: false }));
    }
}

#[derive(Debug)]
pub struct World {
    pub name: String,

    pub data: Vec<Vec<Tile>>,
    pub rows: u32,
    pub columns: u32,

    asset_loader: AssetLoader,
    tile_library: TileLibrary,
    decorations_library: DecorationLibrary
}

impl World {
    pub fn new(name: String, ctx: &mut Context) -> Self {
        let mut asset_loader = AssetLoader::new();

        let tile_library = TileLibrary::new(ctx, &mut asset_loader);
        let decorations_library = DecorationLibrary::new(ctx, &mut asset_loader);

        World {
            name: name,
            data: Vec::new(),
            rows: 10,
            columns: 10,
            asset_loader: asset_loader,
            tile_library: tile_library,
            decorations_library: decorations_library,
        }
    }

    pub fn get_width(&self) -> u32 {
        // Get the width of the world in pixels
        self.columns * TILE_WIDTH
    }

    pub fn get_height(&self) -> u32 {
        // Get the height of the world in pixels
        self.rows * TILE_HEIGHT
    }

    pub fn get_tiles(&self, left: f32, right: f32, top: f32, bottom: f32) -> Vec<Vec<&Tile>> {
        let idx_left = (left / TILE_WIDTH as f32).floor() as usize;
        let mut idx_right = (right / TILE_WIDTH as f32).floor() as usize;
        let idx_top = (top / TILE_HEIGHT as f32).floor() as usize;
        let mut idx_bottom = (bottom / TILE_HEIGHT as f32).floor() as usize;

        if idx_right >= self.columns as usize {
            idx_right = (self.columns - 1) as usize;
        }

        if idx_bottom >= self.rows as usize {
            idx_bottom = (self.rows - 1) as usize;
        }

        // println!("Cam stuff: {} {} {} {}", left, right, top, bottom);
        // println!("Indexes in the world are: Left - {}, Right - {}", idx_left, idx_right);
        // println!("Indexes in the world are: Top - {}, Bottom - {}", idx_top, idx_bottom);

        let mut tmp_rows: Vec<Vec<&Tile>> = Vec::new();
        for i in idx_top..(idx_bottom + 1) {
            let mut tmp_cols: Vec<&Tile> = Vec::new();
            for j in idx_left..(idx_right + 1) {
                tmp_cols.push(&self.data[i][j]);
            }
            tmp_rows.push(tmp_cols);
        }

        tmp_rows
    }

    fn generate_grass_tile(&self, decorations: Option<Vec<Decoration>>) -> Tile {
        let g = self.tile_library.tiles[&TileType::Grass].clone();
        Tile { meta: g, decorations: None}
    }

    fn generate_water_tile(&self) -> Tile {
        let w = self.tile_library.tiles[&TileType::Water].clone();
        Tile { meta: w, decorations: None}
    }

    pub fn load_world_1(&mut self) {
        // Row 1
        let r1 = vec![
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
        ];
        self.data.push(r1);

        // Row 2
        let r2 = vec![
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
        ];
        self.data.push(r2);

        // Row 3
        let b = self.decorations_library.decorations[&DecorationType::Bush1x1].clone();
        let b1 = Decoration {
            flyweight: b,
        };

        let r3 = vec![
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(Some(vec![b1])),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
        ];
        self.data.push(r3);

        // Row 4
        let r4 = vec![
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_water_tile(),
            self.generate_water_tile(),
            self.generate_water_tile(),
            self.generate_water_tile(),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
        ];
        self.data.push(r4);

        // Row 5
        let r5 = vec![
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_water_tile(),
            self.generate_water_tile(),
            self.generate_water_tile(),
            self.generate_water_tile(),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
        ];
        self.data.push(r5);

        // Row 6
        let r6 = vec![
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_water_tile(),
            self.generate_water_tile(),
            self.generate_water_tile(),
            self.generate_water_tile(),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
        ];
        self.data.push(r6);

        // Row 7
        let r7 = vec![
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_water_tile(),
            self.generate_water_tile(),
            self.generate_water_tile(),
            self.generate_water_tile(),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
        ];
        self.data.push(r7);

        // Row 8
        let r8 = vec![
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
        ];
        self.data.push(r8);

        // Row 9
        let r9 = vec![
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
        ];
        self.data.push(r9);

        // Row 10
        let r10 = vec![
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
            self.generate_grass_tile(None),
        ];
        self.data.push(r10);
    }
}
