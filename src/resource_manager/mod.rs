extern crate find_folder;

use std::path::PathBuf;

pub struct Manager {
    assets_path: PathBuf,
}

impl Manager {
    pub fn new() -> Self {
        Manager { assets_path: find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap() }
    }

    pub fn get_asset(&self, name: &str) -> PathBuf {
        self.assets_path.join(name)
    }

    pub fn get_font(&self) -> PathBuf {
        self.get_asset("NotoSans-Regular.ttf")
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
fn get_font() {
    let manager = Manager::new();
    let font_path = manager.get_font();
    assert!(font_path.to_string_lossy().len() > manager.assets_path.to_string_lossy().len());
    assert!(font_path.is_absolute());
    assert!(font_path.is_file());
    assert_eq!("ttf", font_path.extension().unwrap());
}