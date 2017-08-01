extern crate sprite;
extern crate piston_window;
extern crate find_folder;


pub enum Asset {
    Texture<piston_window::Texture>
}


pub struct AssetLoader {
    assets: Vec<Asset>;
    asset_directory: String,
    asset_folder: find_folder::SearchFolder,
}

impl AssetLoader {
    pub fn load_asset(file_name: String, asset_type: Asset) {
        let folder = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();

        folder.join(file_name)

        match asset_type {
            Texture => {
                println!("Loading a texture asset!");
            }
        }
    }
}
