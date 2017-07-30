extern crate piston_window;

use piston_window::{PistonWindow, WindowSettings};

pub mod world;


fn main() {
    let mut world = world::World::generate(String::from("Random world"), 100, 100);
    println!("World: {:#?}", world);

    // Draws a new window
    let mut window: PistonWindow = WindowSettings::new(
        "World Window",
        [800, 800]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    while let Some(event) = window.next() {
        // This runs many times per second...
        window.draw_2d(&event, |context, graphics| {

            piston_window::clear([1.0; 4], graphics);

            let tile_height: f64 = 10.0;
            let tile_width: f64 = 10.0;

            for (i, row) in world.data.iter().enumerate() {
                let i = i as f64;
                for (j, tile) in row.iter().enumerate() {
                    let j = j as f64;
                    piston_window::rectangle(
                        tile.get_color(),
                        [j * tile_width, i * tile_height, tile_height, tile_width],
                        context.transform,
                        graphics
                    );
                }
            }
        });
    }
}
