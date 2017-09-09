use std::rc::Rc;
use std::rc::Weak;

use ggez::graphics::Point;

use rusty_engine::observer::Observable;
use rusty_engine::observer::Observer;

pub struct Player {
    pub point: Point,
    pub is_moveable: bool,
    pub is_invulnerable: bool,

    observables: Vec<Weak<Observable>>
}

impl Player {
    pub fn new() -> Self {
        Player {
            point: Point { x: 0.0, y: 0.0 },
            is_moveable: true,
            is_invulnerable: false,
            observables: Vec::new()
        }
    }
}

impl Observable for Player {
    fn watch(&mut self, observer: Weak<Observer>) {
        println!("Something is watching the player!");
        // self.observables.push(Rc::new(observer));
    }
}
