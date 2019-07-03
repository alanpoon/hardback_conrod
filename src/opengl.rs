use std::collections::HashMap;
use page_curl;
use page_curl::vertex::Vertex;
use backend::meta::app::{ResourceEnum, Sprite, Texture};
use backend::SupportIdType;
use crayon::prelude::*;
use crayon::video::assets::surface::SurfaceHandle;
use crayon::video::assets::shader::ShaderHandle;

/*
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
        let uniforms = uniform! { scale: 10.0f32,tex:texture,rotation:_page.rotation,
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
*/
pub fn draw_multiple(batch: &mut CommandBuffer,
                     vertex_buffer: &[page_curl::page::Vertex],
                     indices: &[u16;859],
                     shader: ShaderHandle,
                     surface: SurfaceHandle,
                     _page_vec: &mut Vec<(page_curl::page::Page, Texture)>,
                     result_map: &HashMap<ResourceEnum, SupportIdType>) {

    for i in (0usize.._page_vec.len()).rev() {
        if let Some(&mut (ref mut _page, ref _sprite)) = _page_vec.get_mut(i) {
            println!("there is page {:?}",_sprite);
            _page.update_time();
            if let Some(&SupportIdType::TextureId(texture)) =
                result_map.get(&ResourceEnum::Texture(_sprite.clone())) {
                println!("texture");
                let mut p_params = MeshParams::default();
                p_params.num_verts = vertex_buffer.len();
                p_params.num_idxes = 859;
                p_params.primitive = MeshPrimitive::TriangleStrip;
                p_params.layout = Vertex::layout();

                let data = MeshData {
                    vptr: Vertex::encode(&vertex_buffer).into(),
                    iptr: IndexFormat::encode(indices).into(),
                };

                let p_mesh = video::create_mesh(p_params, Some(data)).unwrap();
                let mut dc = Draw::new(shader, p_mesh);
                dc.set_uniform_variable("scale", 1.0f32);
                dc.set_uniform_variable("tex", texture);
                dc.set_uniform_variable("rotation", _page.rotation);
                dc.set_uniform_variable("translation", _page.translation);
                dc.set_uniform_variable("theta", _page.theta);
                batch.draw(dc);
                batch.submit(surface).unwrap();

            }
        }
    }

}
