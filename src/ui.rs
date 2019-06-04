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
CGM_iter_resource_enum_vala_pump!{}
pub fn draw(dim:[f64;2],batch:&mut CommandBuffer,renderer: &mut conrod_crayon::Renderer,
            image_map: &conrod_core::image::Map<TextureHandle>,
            primitives: &conrod_core::render::OwnedPrimitives) {
    let dpi_factor = device_pixel_ratio() as f64;
    //let dims = (dim[0] * dpi_factor, dim[1] * dpi_factor);
    renderer.fill((dim[0],dim[1]),dpi_factor, primitives.walk(), &image_map);
    renderer.draw(batch,image_map);
}

pub fn load_resources_iter(map: &mut HashMap<ResourceEnum, Vala>) {
    CGM_iter_resource_enum_vala!{
    (ResourceEnum::Texture(Texture::PAGE1F),"texture","images/characters/player1.jpg"),     
    (ResourceEnum::Texture(Texture::PAGE2F),"texture","images/characters/player2.jpg"),
    (ResourceEnum::Texture(Texture::PAGE3F),"texture","images/characters/player3.jpg"),
    (ResourceEnum::Texture(Texture::PAGE4F),"texture","images/characters/player4.jpg"),
    (ResourceEnum::Sprite(Sprite::KEYPAD),"image","images/keypad.png"),   
    (ResourceEnum::Sprite(Sprite::DOWNLOAD),"image","images/download.png"), //10
    (ResourceEnum::Sprite(Sprite::BACKCARD),"image","images/cards/backside.jpg"),
    (ResourceEnum::Sprite(Sprite::ARROWS),"image","images/arrows_but.png")
    
  }
    iter_resource_enum_vala_new(map)
}

pub fn init_load_resources_to_result_map(result_map: &mut HashMap<ResourceEnum, SupportIdType>,
                                         image_map: &mut conrod_core::image::Map<TextureHandle>,
                                         ui: &mut conrod_core::Ui) {
    CGM_image_map!{
    (ResourceEnum::Sprite(Sprite::RUST),"image","images/rust.png"),
    (ResourceEnum::Sprite(Sprite::UNOFFICIAL),"image","images/unofficial.png"),        
    (ResourceEnum::Sprite(Sprite::CLOUDY),"image","images/cards/cloudy.png"),
    (ResourceEnum::Sprite(Sprite::COININFO),"image","images/allcoin_info.png"),
    (ResourceEnum::Sprite(Sprite::COININFO270),"image","images/allcoin_info (270).png"),
    (ResourceEnum::Sprite(Sprite::GAMEICONS),"image","images/gameicon.png"),
    (ResourceEnum::Font(Font::REGULAR),"font","fonts/NotoSans/NotoSans-Regular.ttf"),
    (ResourceEnum::Font(Font::BOLD),"font","fonts/NotoSans/NotoSans-Bold.ttf"),
    (ResourceEnum::Font(Font::BOLDITALIC),"font","fonts/NotoSans/NotoSans-BoldItalic.ttf"),
    (ResourceEnum::Font(Font::ITALIC),"font","fonts/NotoSans/NotoSans-Italic.ttf"),
    (ResourceEnum::Font(Font::MYSTERY),"font","fonts/MysteryQuest-Regular.ttf"),
    (ResourceEnum::Font(Font::HORROR),"font","fonts/Mortified.ttf"),
    (ResourceEnum::Font(Font::ADVENTURE),"font","fonts/TradeWinds-Regular.ttf"),
    (ResourceEnum::Font(Font::ROMANCE),"font","fonts/Babylove.ttf"),
    //(ResourceEnum::Music(MusicEnum::BACKGROUND),"music","audio/doki1.ogg"),
    //(ResourceEnum::Chunk(ChunkEnum::PAGEFLIP),"chunk","audio/Page_urn_sound_effect.ogg")
    
  }
    let g = ImageIds::new();
    g.pump(result_map, ui, image_map);
}
pub const RESULTMAPLEN: usize = 21;
//don't count music
