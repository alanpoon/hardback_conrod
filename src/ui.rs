use conrod;
use conrod::backend::glium::glium::{self, Surface};
use backend::SupportIdType;
use backend::meta::app::{Font, ResourceEnum};
use image;
use find_folder;
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
                                    image_map: &mut conrod::image::Map<glium::texture::Texture2d>,
                                    display: &glium::Display,
                                    ui: &mut conrod::Ui) {
    CGM_image_map!{
    (ResourceEnum::Font(Font::REGULAR),"font","fonts/NotoSans/NotoSans-Regular.ttf"),
     (ResourceEnum::Font(Font::BEON),"font","fonts/Beon/beon-webfont.ttf")
  }
    let g = ImageIds::new();
    g.pump(result_map, display, ui, image_map);
}
