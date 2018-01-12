use conrod;
use conrod::backend::glium::glium::{self, Surface};
use backend::SupportIdType;
use backend::meta::app::{self, Font, ResourceEnum, Sprite, Texture, ChunkEnum, MusicEnum};
use support;
use image;
use rodio;
use std::collections::HashMap;
#[derive(Clone)]
pub struct Vala {
    source_type: &'static str,
    path: &'static str,
}
CGM_iter_resource_enum_vala_pump!{}
pub fn draw(display: &glium::Display,
            renderer: &mut conrod::backend::glium::Renderer,
            image_map: &conrod::image::Map<glium::Texture2d>,
            primitives: &conrod::render::OwnedPrimitives) {
    renderer.fill(display, primitives.walk(), &image_map);
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 1.0);
    renderer.draw(display, &mut target, &image_map).unwrap();
    target.finish().unwrap();
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
                                         image_map: &mut conrod::image::Map<glium::Texture2d>,
                                         display: &glium::Display,
                                         ui: &mut conrod::Ui) {
    CGM_image_map!{
    (ResourceEnum::Sprite(Sprite::RUST),"image","images/rust.png"),
    (ResourceEnum::Sprite(Sprite::UNOFFICIAL),"image","images/unofficial.png"),        
    (ResourceEnum::Sprite(Sprite::CLOUDY),"image","images/cards/cloudy.png"),
    (ResourceEnum::Sprite(Sprite::COININFO),"image","images/allcoin_info.png"),
    (ResourceEnum::Sprite(Sprite::COININFO270),"image270","images/allcoin_info.png"),
    (ResourceEnum::Sprite(Sprite::GAMEICONS),"image","images/gameicon.png"),
    (ResourceEnum::Font(Font::REGULAR),"font","fonts/NotoSans/NotoSans-Regular.ttf"),
    (ResourceEnum::Font(Font::BOLD),"font","fonts/NotoSans/NotoSans-Bold.ttf"),
    (ResourceEnum::Font(Font::BOLDITALIC),"font","fonts/NotoSans/NotoSans-BoldItalic.ttf"),
    (ResourceEnum::Font(Font::ITALIC),"font","fonts/NotoSans/NotoSans-Italic.ttf"),
    (ResourceEnum::Font(Font::MYSTERY),"font","fonts/MysteryQuest-Regular.ttf"),
    (ResourceEnum::Font(Font::HORROR),"font","fonts/Mortified.ttf"),
    (ResourceEnum::Font(Font::ADVENTURE),"font","fonts/TradeWinds-Regular.ttf"),
    (ResourceEnum::Font(Font::ROMANCE),"font","fonts/Babylove.ttf"),
   // (ResourceEnum::Music(MusicEnum::BACKGROUND),"music","audio/doki1.ogg"),
    //(ResourceEnum::Chunk(ChunkEnum::PAGEFLIP),"chunk","audio/Page_urn_sound_effect.ogg")
    
  }
    let g = ImageIds::new();
    g.pump(result_map, display, ui, image_map);
}
pub const RESULTMAPLEN: usize = 21;
//don't count music
