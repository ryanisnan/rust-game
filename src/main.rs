extern crate piston_window;

use piston_window::*;

pub mod world;
pub mod camera;

fn main() {
    let mut world = world::World::generate_world_1();
    // println!("World: {:#?}", world);

    // Draws a new window
    let mut window: PistonWindow = WindowSettings::new(
        "World Window",
        [800, 800]
    )
    .exit_on_esc(true)
    .samples(4)
    .build()
    .unwrap();

    let mut cam = camera::Camera::new(800, 800, world.get_width(), world.get_height());

    while let Some(event) = window.next() {
        match event.press_args() {
            Some(piston_window::Button::Keyboard(piston_window::Key::Left)) => {
                cam.move_left();
            },
            Some(piston_window::Button::Keyboard(piston_window::Key::Right)) => {
                cam.move_right();
            },
            Some(piston_window::Button::Keyboard(piston_window::Key::Up)) => {
                cam.move_up();
            },
            Some(piston_window::Button::Keyboard(piston_window::Key::Down)) => {
                cam.move_down();
            },
            Some(_) => (),
            None => ()
        }

        let tiles = world.get_tiles(cam.get_left(), cam.get_right(), cam.get_top(), cam.get_bottom());

        let x_offset = cam.get_left() % world.tile_width;
        let y_offset = cam.get_top() % world.tile_height;

        // println!("{:#?}", event);
        // This runs many times per second...
        window.draw_2d(&event, |context, graphics| {

            piston_window::clear([1.0; 4], graphics);

            for (i, row) in tiles.iter().enumerate() {
                let i = i as f64;
                for (j, tile) in row.iter().enumerate() {
                    let j = j as f64;
                    piston_window::rectangle(
                        tile.get_color(),
                        [j * world.tile_width - x_offset, i * world.tile_height - y_offset, world.tile_height, world.tile_width],
                        context.transform,
                        graphics
                    );
                }
            }
        });
    }
}
