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
    pub height: u8,
    pub width: u8
}

impl World {
    pub fn generate(name: String, height: u8, width: u8) -> World {
        let mut data: Vec<Vec<Tile>> = Vec::new();
        for i in 0..height {
            let mut row: Vec<Tile> = Vec::new();
            for j in 0..width {
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
            height,
            width,
        }
    }
}
