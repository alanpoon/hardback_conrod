use conrod_core;
use conrod_crayon;
use backend::SupportIdType;
use backend::meta::app::{self, Font, ResourceEnum, Sprite, Texture, ChunkEnum, MusicEnum};
use support;
use image;
use rodio;
use std::collections::HashMap;
use crayon::prelude::*;
use crayon_audio::prelude::*;
use crayon_bytes::prelude::*;
use crayon::window::device_pixel_ratio;
#[derive(Clone)]
pub struct Vala {
    source_type: &'static str,
    path: &'static str,
}

pub fn draw(dim:[f64;2],batch:&mut CommandBuffer,renderer: &mut conrod_crayon::Renderer,
            image_map: &conrod_core::image::Map<TextureHandle>,
            primitives: &conrod_core::render::OwnedPrimitives) {
    let dpi_factor = device_pixel_ratio() as f64;
    //let dims = (dim[0] * dpi_factor, dim[1] * dpi_factor);
    renderer.fill((dim[0],dim[1]),dpi_factor, primitives.walk(), &image_map);
    renderer.draw(batch,image_map);
}

pub const RESULTMAPLEN: usize = 20;
//don't count music
