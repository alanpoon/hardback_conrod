pub mod page;
pub mod vertex;

pub fn render(_page:&mut page::Page)->&Vec<vertex::Vertex>{
     _page.flip();
     _page.create_mesh();
     &_page.out_mesh
}
