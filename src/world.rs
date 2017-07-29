extern crate rand;


#[derive(Debug)]
pub enum Tile {
    Grass,
    Water
}

impl Tile {
    pub fn get_color(&self) -> (u8, u8, u8, f64) {  // r, g, b, a
        match *self {
            Tile::Grass => (0, 255, 0, 1.0),
            Tile::Water => (0, 0, 255, 1.0),
        }
    }

    pub fn is_walkable(&self) -> bool {
        match *self {
            Tile::Grass => true,
            Tile::Water => false,
        }
    }
}

#[derive(Debug)]
pub struct World {
    name: String,
    data: Vec<Vec<Tile>>,
    height: u8,
    width: u8
}

impl World {
    pub fn generate(name: String, height: u8, width: u8) -> World {
        let mut data: Vec<Vec<Tile>> = Vec::new();
        for i in (0..height) {
            let mut row: Vec<Tile> = Vec::new();
            for j in (0..width) {
                let x = rand::random::<f64>();
                if x > 0.5 {
                    row.push(Tile::Grass);
                } else {
                    row.push(Tile::Water);
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
