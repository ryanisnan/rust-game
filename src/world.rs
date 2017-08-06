extern crate rand;
extern crate ggez;

use assets::AssetLoader;
use ggez::graphics::Image;
use ggez::Context;


#[derive(Debug)]
pub enum Tile {
    Grass,
    Water,
    Stone
}

impl Tile {
    pub fn get_color(&self) -> [f32; 4] {  // r, g, b, a
        match *self {
            Tile::Grass => [0.0, 1.0, 0.0, 1.0],
            Tile::Water => [0.0, 0.0, 1.0, 1.0],
            Tile::Stone => [0.6, 0.6, 0.6, 1.0]
        }
    }

    pub fn get_image(&self, ctx: &mut Context, asset_loader: &mut AssetLoader) -> &Image {
        match *self {
            Tile::Grass => {
                let foo = asset_loader.load_image(ctx, "/grass-1.png");
                *foo
            },
            Tile::Water => {
                &asset_loader.load_image(ctx, "/water-1.png")
            },
            Tile::Stone => {
                &asset_loader.load_image(ctx, "/grass-1.png")
            }
        }
    }

    pub fn is_walkable(&self) -> bool {
        match *self {
            Tile::Grass => true,
            Tile::Water => false,
            Tile::Stone => false,
        }
    }
}

#[derive(Debug)]
pub struct World {
    pub name: String,
    pub data: Vec<Vec<Tile>>,
    pub rows: u32,
    pub columns: u32,
    pub tile_height: f32,
    pub tile_width: f32,
}

impl World {
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

    pub fn generate_random_world(name: String, rows: u32, columns: u32) -> World {
        let mut data: Vec<Vec<Tile>> = Vec::new();
        for i in 0..rows {
            let mut row: Vec<Tile> = Vec::new();
            for j in 0..columns {
                let x = rand::random::<f32>();
                if x >= 0.25 {
                    row.push(Tile::Grass);
                } else if x >= 0.05 {
                    row.push(Tile::Water);
                } else {
                    row.push(Tile::Stone);
                }
            }
            data.push(row);
        }
        World {
            name,
            data,
            rows,
            columns,
            tile_height: 50.0,
            tile_width: 50.0,
        }
    }

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

        data.push(vec![
            Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass, Tile::Grass
        ]);



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
