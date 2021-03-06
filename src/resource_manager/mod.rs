use std::env::current_exe;
use std::path::PathBuf;
use graphics::Texture;
use piston_window::{PistonWindow, Flip, TextureSettings};

pub enum Textures {
    EmptyTile,
}

pub struct Manager {
    assets_path: PathBuf,
}

impl Manager {
    pub fn new() -> Self {
        let mut path = current_exe().expect("Unable to get 'current_exe' path");
        path.pop(); // Executable name.
        path.pop(); // 'debug'/'release' folder.
        path.pop(); // 'target' folder.

        Manager { assets_path: path.join("assets") }
    }

    pub fn get_asset(&self, name: &str) -> PathBuf {
        self.assets_path.join(name)
    }

    pub fn get_font(&self) -> PathBuf {
        self.get_asset("NotoSans-Regular.ttf")
    }

    pub fn load_texture(&self, window: &PistonWindow, texture: Textures) -> Texture {
        Texture::from_path(&mut *window.factory.borrow_mut(),
                           self.get_asset(texture_to_str(texture)),
                           Flip::None,
                           &TextureSettings::new())
            .unwrap()
    }
}

fn texture_to_str(texture: Textures) -> &'static str {
    match texture {
        Textures::EmptyTile => "empty_tile.png",
    }
}

#[test]
fn new() {
    let manager = Manager::new();
    assert!(!manager.assets_path.to_string_lossy().is_empty());
    assert!(manager.assets_path.is_absolute());
    assert!(manager.assets_path.exists());
    assert!(manager.assets_path.is_dir());
}

#[test]
fn get_asset() {
    let manager = Manager::new();
    let asset_path = manager.get_asset("test");
    assert!(asset_path.to_string_lossy().len() > manager.assets_path.to_string_lossy().len());
}

#[test]
fn get_font() {
    let manager = Manager::new();
    let font_path = manager.get_font();
    assert!(font_path.is_absolute());
    assert!(font_path.is_file());
    assert_eq!("ttf", font_path.extension().unwrap());
}
