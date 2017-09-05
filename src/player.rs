use ggez::graphics::Point;

use rusty_engine::observer::Observable;
use rusty_engine::observer::Observer;

pub struct Player {
    pub point: Point,
    pub is_moveable: bool,
    pub is_invulnerable: bool,
}

impl Observable for Player {
    fn watch(&self, observer: &Observer) {
        println!("Something is watching the player!");
    }
}
