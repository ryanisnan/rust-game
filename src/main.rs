mod world;

fn main() {
    let world = world::World::generate(String::from("Random world"), 4, 4);
    println!("World: {:#?}", world);
}
