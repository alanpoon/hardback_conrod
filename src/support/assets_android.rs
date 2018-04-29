use android_glue;
use image::{self, DynamicImage};
use rusttype::{Font, FontCollection};
pub fn load_font(filename: &str) -> Font {
    match android_glue::load_asset(filename) {
        Ok(data) => FontCollection::from_bytes(data).unwrap().into_font().unwrap(),
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
pub type AudioType = i16;
pub fn load_audio(filename: &str) -> AudioType {
    match android_glue::load_asset(filename) {
        Ok(data) => 2,
        Err(_) => panic!("Can't load music."),
    }
}
