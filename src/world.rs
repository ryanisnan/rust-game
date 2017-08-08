extern crate rand;
extern crate ggez;

use assets::AssetLoader;
use ggez::graphics::Image;
use ggez::Context;
use entity::DecorationLibrary;

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
    pub x: u32,
    pub y: u32,
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
            rows: 8,
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

    pub fn load_world_1(&mut self) {
        let g = self.tile_library.tiles[&TileType::Grass].clone();
        let w = self.tile_library.tiles[&TileType::Water].clone();

        // Row 1
        self.data.push(vec![
            Tile { meta: g.clone(), x: 0, y: 0},
            Tile { meta: g.clone(), x: 1, y: 0},
            Tile { meta: g.clone(), x: 2, y: 0},
            Tile { meta: g.clone(), x: 3, y: 0},
            Tile { meta: g.clone(), x: 4, y: 0},
            Tile { meta: g.clone(), x: 5, y: 0},
            Tile { meta: g.clone(), x: 6, y: 0},
            Tile { meta: g.clone(), x: 7, y: 0},
            Tile { meta: g.clone(), x: 8, y: 0},
            Tile { meta: g.clone(), x: 9, y: 0},
        ]);

        // Row 2
        self.data.push(vec![
            Tile { meta: g.clone(), x: 0, y: 1},
            Tile { meta: g.clone(), x: 1, y: 1},
            Tile { meta: g.clone(), x: 2, y: 1},
            Tile { meta: g.clone(), x: 3, y: 1},
            Tile { meta: g.clone(), x: 4, y: 1},
            Tile { meta: g.clone(), x: 5, y: 1},
            Tile { meta: g.clone(), x: 6, y: 1},
            Tile { meta: g.clone(), x: 7, y: 1},
            Tile { meta: g.clone(), x: 8, y: 1},
            Tile { meta: g.clone(), x: 9, y: 1},
        ]);

        // Row 3
        self.data.push(vec![
            Tile { meta: g.clone(), x: 0, y: 2},
            Tile { meta: g.clone(), x: 1, y: 2},
            Tile { meta: g.clone(), x: 2, y: 2},
            Tile { meta: w.clone(), x: 3, y: 2},
            Tile { meta: w.clone(), x: 4, y: 2},
            Tile { meta: w.clone(), x: 5, y: 2},
            Tile { meta: w.clone(), x: 6, y: 2},
            Tile { meta: g.clone(), x: 7, y: 2},
            Tile { meta: g.clone(), x: 8, y: 2},
            Tile { meta: g.clone(), x: 9, y: 2},
        ]);

        // Row 4
        self.data.push(vec![
            Tile { meta: g.clone(), x: 0, y: 3},
            Tile { meta: g.clone(), x: 1, y: 3},
            Tile { meta: g.clone(), x: 2, y: 3},
            Tile { meta: w.clone(), x: 3, y: 3},
            Tile { meta: w.clone(), x: 4, y: 3},
            Tile { meta: w.clone(), x: 5, y: 3},
            Tile { meta: w.clone(), x: 6, y: 3},
            Tile { meta: g.clone(), x: 7, y: 3},
            Tile { meta: g.clone(), x: 8, y: 3},
            Tile { meta: g.clone(), x: 9, y: 3},
        ]);

        // Row 5
        self.data.push(vec![
            Tile { meta: g.clone(), x: 0, y: 4},
            Tile { meta: g.clone(), x: 1, y: 4},
            Tile { meta: g.clone(), x: 2, y: 4},
            Tile { meta: w.clone(), x: 3, y: 4},
            Tile { meta: w.clone(), x: 4, y: 4},
            Tile { meta: w.clone(), x: 5, y: 4},
            Tile { meta: w.clone(), x: 6, y: 4},
            Tile { meta: g.clone(), x: 7, y: 4},
            Tile { meta: g.clone(), x: 8, y: 4},
            Tile { meta: g.clone(), x: 9, y: 4},
        ]);

        // Row 6
        self.data.push(vec![
            Tile { meta: g.clone(), x: 0, y: 5},
            Tile { meta: g.clone(), x: 1, y: 5},
            Tile { meta: g.clone(), x: 2, y: 5},
            Tile { meta: g.clone(), x: 3, y: 5},
            Tile { meta: g.clone(), x: 4, y: 5},
            Tile { meta: g.clone(), x: 5, y: 5},
            Tile { meta: g.clone(), x: 6, y: 5},
            Tile { meta: g.clone(), x: 7, y: 5},
            Tile { meta: g.clone(), x: 8, y: 5},
            Tile { meta: g.clone(), x: 9, y: 5},
        ]);

        // Row 7
        self.data.push(vec![
            Tile { meta: g.clone(), x: 0, y: 6},
            Tile { meta: g.clone(), x: 1, y: 6},
            Tile { meta: g.clone(), x: 2, y: 6},
            Tile { meta: g.clone(), x: 3, y: 6},
            Tile { meta: g.clone(), x: 4, y: 6},
            Tile { meta: g.clone(), x: 5, y: 6},
            Tile { meta: g.clone(), x: 6, y: 6},
            Tile { meta: g.clone(), x: 7, y: 6},
            Tile { meta: g.clone(), x: 8, y: 6},
            Tile { meta: g.clone(), x: 9, y: 6},
        ]);

        // Row 8
        self.data.push(vec![
            Tile { meta: g.clone(), x: 0, y: 7},
            Tile { meta: g.clone(), x: 1, y: 7},
            Tile { meta: g.clone(), x: 2, y: 7},
            Tile { meta: g.clone(), x: 3, y: 7},
            Tile { meta: g.clone(), x: 4, y: 7},
            Tile { meta: g.clone(), x: 5, y: 7},
            Tile { meta: g.clone(), x: 6, y: 7},
            Tile { meta: g.clone(), x: 7, y: 7},
            Tile { meta: g.clone(), x: 8, y: 7},
            Tile { meta: g.clone(), x: 9, y: 7},
        ]);
    }
}
