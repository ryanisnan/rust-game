use ggez::graphics::Point;

use rusty_engine::entity::Entity;
use rusty_engine::physics::Direction;
use rusty_engine::physics::MovePhysicsAction;

#[derive(Debug)]
pub struct Player {
    pub point: Point,
    pub is_moveable: bool,
    pub is_invulnerable: bool,

    base_move_speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            point: Point { x: 0.0, y: 0.0 },
            is_moveable: true,
            is_invulnerable: false,
            base_move_speed: 10.0,
        }
    }

    pub fn generate_move_up_action(&self) -> MovePhysicsAction {
        MovePhysicsAction {
            direction: Direction::Up,
            distance: self.base_move_speed,
        }
    }
}

impl Entity for Player {
    fn is_moveable(&self) -> bool { true }
    fn is_visible(&self) -> bool { true }

    // Commands for forecfully moving the player
    fn move_up(&mut self, distance: f32) -> () {
        self.point.y += distance;
    }

    fn move_down(&mut self, distance: f32) -> () {
        self.point.y -= distance;
    }

    fn move_left(&mut self, distance: f32) -> () {
        self.point.x -= distance;
    }

    fn move_right(&mut self, distance: f32) -> () {
        self.point.x += distance;
    }
}
