use conrod;
use conrod::backend::glium::glium::{self, Surface};
use backend::SupportIdType;
use backend::meta::app::{self, Font, ResourceEnum, Sprite};
use support;
use std::collections::HashMap;
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

pub fn load_resources_to_result_map(result_map: &mut HashMap<ResourceEnum, SupportIdType>,
                                    image_map: &mut conrod::image::Map<glium::Texture2d>,
                                    display: &glium::Display,
                                    ui: &mut conrod::Ui) {
    CGM_image_map!{
    (ResourceEnum::Sprite(Sprite::BUTTON),"image",app::get_button_path()),
    (ResourceEnum::Sprite(Sprite::PAGE1F),"texture","images/characters/player1.jpg"),     
    (ResourceEnum::Sprite(Sprite::PAGE1F),"texture","images/characters/player1.jpg"),
    (ResourceEnum::Sprite(Sprite::PAGE2F),"texture","images/characters/player2.jpg"),
    (ResourceEnum::Sprite(Sprite::PAGE3F),"texture","images/characters/player3.jpg"),
    (ResourceEnum::Sprite(Sprite::PAGE4F),"texture","images/characters/player4.jpg"),
    (ResourceEnum::Sprite(Sprite::RUST),"image","images/rust.png"),
    (ResourceEnum::Sprite(Sprite::KEYPAD),"image","images/keypad.png"),
    (ResourceEnum::Sprite(Sprite::CARDS1),"image","images/cards/p01-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS2),"image","images/cards/p02-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS3),"image","images/cards/p03-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS4),"image","images/cards/p04-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS5),"image","images/cards/p05-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS6),"image","images/cards/p06-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS7),"image","images/cards/p07-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS8),"image","images/cards/p08-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS9),"image","images/cards/p09-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS10),"image","images/cards/p10-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS11),"image","images/cards/p11-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS12),"image","images/cards/p12-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS13),"image","images/cards/p13-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS14),"image","images/cards/p14-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS15),"image","images/cards/p15-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS16),"image","images/cards/p16-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS17),"image","images/cards/p17-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS18),"image","images/cards/p18-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS19),"image","images/cards/p19-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS20),"image","images/cards/p20-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS21),"image270","images/cards/p03-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS22),"image270","images/cards/p06-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS23),"image270","images/cards/p07-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS24),"image270","images/cards/p10-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS25),"image270","images/cards/p11-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS26),"image270","images/cards/p14-1.jpg"),
    (ResourceEnum::Sprite(Sprite::CARDS27),"image270","images/cards/p15-1.jpg"),
    (ResourceEnum::Sprite(Sprite::DOWNLOAD),"image","images/download.png"),
    (ResourceEnum::Sprite(Sprite::BACKCARD),"image","images/cards/backside.jpg"),
    (ResourceEnum::Font(Font::REGULAR),"font","fonts/NotoSans/NotoSans-Regular.ttf"),
    // (ResourceEnum::Font(Font::BEON),"font","fonts/Beon/beon-webfont.ttf")
  }
    let g = ImageIds::new();
    g.pump(result_map, display, ui, image_map);
}
