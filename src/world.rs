extern crate rand;


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
    pub tile_height: f64,
    pub tile_width: f64,
}

impl World {
    pub fn generate(name: String, rows: u32, columns: u32) -> World {
        let mut data: Vec<Vec<Tile>> = Vec::new();
        for i in 0..rows {
            let mut row: Vec<Tile> = Vec::new();
            for j in 0..columns {
                let x = rand::random::<f64>();
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

    pub fn get_tiles(&self, left: f64, right: f64, top: f64, bottom: f64) -> Vec<Vec<&Tile>> {
        let idx_left = (left / self.tile_width).floor() as usize;
        let idx_right = (right / self.tile_width).ceil() as usize;
        let idx_top = (top / self.tile_height).floor() as usize;
        let idx_bottom = (bottom / self.tile_height).ceil() as usize;

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
