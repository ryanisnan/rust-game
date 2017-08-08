use std::collections::HashMap;
use std::rc::Rc;
use ggez::Context;
use ggez::graphics::Image;
use assets::AssetLoader;

pub trait Entity {} // Things that can be placed into the world will all implement this functionality

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum DecorationType {
    Bush1x1,
}

#[derive(Debug)]
pub struct Decoration {
    image: Rc<Image>
}

#[derive(Debug)]
pub struct DecorationLibrary {
    pub decorations: HashMap<DecorationType, Rc<Decoration>>
}

impl DecorationLibrary {
    pub fn new(ctx: &mut Context, asset_loader: &mut AssetLoader) -> Self {
        let mut dl = DecorationLibrary {
            decorations: HashMap::new(),
        };

        dl.load_assets(ctx, asset_loader);

        dl
    }

    fn load_assets(&mut self, ctx: &mut Context, asset_loader: &mut AssetLoader) {
        self.decorations.insert(DecorationType::Bush1x1, Rc::new(Decoration { image: asset_loader.load_image(ctx, "/bush-1.png")}));
    }
}
