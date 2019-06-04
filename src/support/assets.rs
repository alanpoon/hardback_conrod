use image::{self, DynamicImage};
use conrod_core::text::{font, Font};
use find_folder;
use std;
use std::io::BufReader;
use std::io::Read;
pub fn load_font(filename: &str) -> Font {
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let path = assets.join(filename);
    match font::from_file(path) {
        Ok(data) => data,
        Err(_) => panic!("Can't load font."),
    }
}

pub fn load_image(filename: &str) -> DynamicImage {
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let path = assets.join(filename);
    match image::open(&std::path::Path::new(&path)) {
        Ok(data) => data,
        Err(_) => panic!("Can't load image."),
    }
}
pub fn load_90image(filename: &str) -> DynamicImage {
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let path = assets.join(filename);
    match image::open(&std::path::Path::new(&path)) {
        Ok(data) => data.rotate90(),
        Err(_) => panic!("Can't load image."),
    }
}
pub fn load_270image(filename: &str) -> DynamicImage {
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let path = assets.join(filename);
    match image::open(&std::path::Path::new(&path)) {
        Ok(data) => data.rotate270(),
        Err(_) => panic!("Can't load image."),
    }
}
pub type AudioType = i16;
pub fn load_audio(filename: &str) -> AudioType {
    match std::fs::File::open(filename) {
        Ok(mut file) => 2,
        Err(_) => panic!("Can't load music."),
    }
}
