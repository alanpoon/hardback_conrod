use image::{self, DynamicImage};
use sdl2::mixer::{Music, Chunk};
use conrod::text::{font, Font};
use find_folder;
use std;
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
pub fn load_music(filename: &str) -> Music<'static> {
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let path = assets.join(filename);
    match Music::from_file(&std::path::Path::new(&path)) {
        Ok(data) => data,
        Err(_) => panic!("Can't load music."),
    }
}
/*
pub fn load_chunk(filename: &str) -> Chunk {
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let path = assets.join(filename);
    match Chunk::from_file(&std::path::Path::new(&path)) {
        Ok(data) => data,
        Err(er) => {
            println!("er {:?}",er);
            panic!("Can't load chunk.");
        }
    }
}
*/