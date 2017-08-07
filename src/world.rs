extern crate rand;
extern crate ggez;

use assets::AssetLoader;
use ggez::graphics::Image;
use ggez::Context;

use std::collections::HashMap;


enum TileType {
    // Basic enum listing the various types of tiles within the world
    Grass,
    Water
}

struct TileMeta {
    // Defines data common across various types of tiles
    // These instance will be referenced by other world tiles
    image: &Image,
    is_walkable: bool
}

struct Tile {
    // Represents a game tile in the world
    meta: &TileMeta,
    x: u32,
    y: u32,
}

#[derive(Debug)]
pub struct World {
    pub name: String,

    pub data: Vec<Vec<Tile>>,
    pub rows: u32,
    pub columns: u32,
    pub tile_height: f32,
    pub tile_width: f32,

    assets: AssetLoader,
    tile_types: HashMap<TileType, TileMeta>
}

impl World {
    pub fn new(name: &String) -> Self {
        // TODO: Implement me!
    }

    fn load_tile_meta(&mut self, ctx: &mut Context) {
        // Populate TileMeta instances into the local TileMeta hashmap
        self.tile_types.insert(TileType::Grass, TileMeta { image: self.assets.load_image(ctx, "/grass-1.png"), is_walkable: true });
        self.tile_types.insert(TileType::Water, TileMeta { image: self.assets.load_image(ctx, "/water-3.png"), is_walkable: false });
    }

    pub fn new() -> Self {
        return World {}
    }

    pub fn get_width(&self) -> f32 {
        self.columns as f32 * self.tile_width
    }

    pub fn get_height(&self) -> f32 {
        self.rows as f32 * self.tile_height
    }

    pub fn get_tiles(&self, left: f32, right: f32, top: f32, bottom: f32) -> Vec<Vec<&Tile>> {
        let idx_left = (left / self.tile_width).floor() as usize;
        let mut idx_right = (right / self.tile_width).floor() as usize;
        let idx_top = (top / self.tile_height).floor() as usize;
        let mut idx_bottom = (bottom / self.tile_height).floor() as usize;

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
}

impl World {


    pub fn generate_world_1() -> World {
        let mut data: Vec<Vec<Tile>> = Vec::new();

        data.push(vec![Tile::Stone, Tile::Stone, Tile::Stone, Tile::Stone, Tile::Stone, Tile::Stone, Tile::Stone, Tile::Stone, Tile::Stone, Tile::Stone]);
        data.push(vec![Tile::Stone, Tile::Stone, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass]);
        data.push(vec![Tile::Stone, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass]);
        data.push(vec![Tile::Stone, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Water, Tile::Water, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass]);
        data.push(vec![Tile::Stone, Tile::Grass, Tile::Water, Tile::Water, Tile::Water, Tile::Water, Tile::Water, Tile::Water, Tile::Grass, Tile::Grass]);
        data.push(vec![Tile::Stone, Tile::Grass, Tile::Water, Tile::Water, Tile::Water, Tile::Water, Tile::Water, Tile::Water, Tile::Grass, Tile::Grass]);
        data.push(vec![Tile::Stone, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Water, Tile::Water, Tile::Water, Tile::Water, Tile::Grass, Tile::Grass]);
        data.push(vec![Tile::Stone, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Stone]);
        data.push(vec![Tile::Stone, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Stone]);
        data.push(vec![Tile::Grass, Tile::Grass, Tile::Stone, Tile::Stone, Tile::Stone, Tile::Stone, Tile::Stone, Tile::Stone, Tile::Stone, Tile::Stone]);

        World {
            name: String::from("World 1"),
            rows: 10,
            columns: 10,
            tile_height: 64.0,
            tile_width: 64.0,
            data: data,
        }
    }
}
