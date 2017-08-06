extern crate ggez;

use ggez::Context;

pub trait Loadable {
    fn load(ctx: &mut Context, file_path: &str) -> Self;
}

#[derive(Debug)]
pub enum AssetType {
    Image(ggez::graphics::Image),
}

impl Loadable for ggez::graphics::Image {
    fn load(ctx: &mut Context, file_path: &str) -> Self {
        println!("Loading an image...");
        ggez::graphics::Image::new(ctx, file_path).unwrap()
    }
}

pub struct AssetLoader {
    assets: Vec<Box<AssetType>>,
}

impl AssetLoader {
    pub fn new() -> Self {
        AssetLoader {
            assets: Vec::<Box<AssetType>>::new()
        }
    }

    pub fn load<T: Loadable>(&self, ctx: &mut Context, file_path: &str) {
        println!("AssetLoader: Loading an asset...");

        self.assets.push(Box::new(T::load(ctx, file_path)));
    }
}
