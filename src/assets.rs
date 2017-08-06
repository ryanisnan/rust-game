extern crate ggez;

use ggez::Context;
use ggez::graphics::Image;
use std::sync::Arc;
use std::any::Any;
use std::collections::HashMap;


pub trait Loadable {
    fn load(ctx: &mut Context, file_path: &str) -> Self where Self: Sized;
}

impl Loadable for ggez::graphics::Image {
    fn load(ctx: &mut Context, file_path: &str) -> Self {
        println!("Loading an image...");
        ggez::graphics::Image::new(ctx, file_path).unwrap()
    }
}

pub struct AssetLoader {
    assets: HashMap<String, Box<Any>>,
}

impl AssetLoader {
    /*
        Loads assets and stores them in a hashmap with the Key being the path for the file,
        and the Value is the asset content.
    */
    pub fn new() -> Self {
        AssetLoader {
            assets: HashMap::new(),
        }
    }

    pub fn load<T: Loadable + 'static>(&mut self, ctx: &mut Context, file_path: &str) -> Arc<T> {
        println!("AssetLoader: Loading an asset...");

        let asset = self.assets.entry(file_path.into()).or_insert_with(|| Box::new(Arc::new(T::load(ctx, file_path))));

        if !asset.is::<T>() {
            println!("Warning: Loading file as different asset type");
            *asset = Box::new(Arc::new(T::load(ctx, file_path)));
        }

        asset.downcast_ref::<Arc<T>>().unwrap().clone()
    }

    pub fn load_image(&mut self, ctx: &mut Context, path: &str) -> Arc<Image> {
        self.load(ctx, path)
    }
}
