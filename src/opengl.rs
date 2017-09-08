use glium;
use glium::Surface;
use std::collections::HashMap;
use page_curl;
use backend::meta::app::{ResourceEnum, Sprite};
use backend::SupportIdType;
pub fn draw(display: &glium::Display,
            vertex_buffer: &glium::VertexBuffer<page_curl::page::Vertex>,
            indices: &glium::IndexBuffer<u16>,
            program: &glium::Program,
            _page: &mut page_curl::page::Page,
            result_map: &HashMap<ResourceEnum, SupportIdType>) {
    _page.update_time();

    let mut target = display.draw();
    if let Some(&SupportIdType::TextureId(ref texture)) =
        result_map.get(&ResourceEnum::Sprite(Sprite::BUTTON)) {
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        let uniforms = uniform! { scale: 1.0f32,tex:texture,rotation:_page.rotation,
                translation:_page.translation,
                theta:_page.theta };
        target.draw(vertex_buffer,
                    indices,
                    program,
                    &uniforms,
                    &Default::default())
            .unwrap();
        target.finish().unwrap();
    }

}
