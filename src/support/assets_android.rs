use android_glue;
use image::{self, DynamicImage};
use rusttype::{Font, FontCollection};
use sdl2::mixer::{Music, Chunk};
pub fn load_font(filename: &str) -> Font {
    match android_glue::load_asset(filename) {
        Ok(data) => FontCollection::from_bytes(data).into_font().unwrap(),
        Err(_) => panic!("Can't load font."),
    }
}

pub fn load_image(filename: &str) -> DynamicImage {
    match android_glue::load_asset(filename) {
        Ok(data) => image::load_from_memory(&data).unwrap(),
        Err(_) => panic!("Can't load image."),
    }
}
pub fn load_90image(filename: &str) -> DynamicImage {
    match android_glue::load_asset(filename) {
        Ok(data) => image::load_from_memory(&data).unwrap().rotate90(),
        Err(_) => panic!("Can't load image."),
    }
}
pub fn load_270image(filename: &str) -> DynamicImage {
    match android_glue::load_asset(filename) {
        Ok(data) => image::load_from_memory(&data).unwrap().rotate270(),
        Err(_) => panic!("Can't load image."),
    }
}
pub fn load_music(filename: &str) -> Music<'static> {
    match android_glue::load_asset(filename) {
        Ok(data) =>  Music::from_static_bytes(data),
        Err(_) => panic!("Can't load music."),
    }
}
/*
pub fn load_chunk(filename: &str) -> Chunk {
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let path = assets.join(filename);
    match android_glue::load_asset(filename)Chunk::from_file(&std::path::Path::new(&path)) {
        Ok(data) => data,
        Err(_) => panic!("Can't load chunk."),
    }
}
*/