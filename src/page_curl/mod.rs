pub mod page;
pub mod vertex;
pub fn render(_page: &mut page::Page) {
    _page.flip();
    _page.create_mesh();
}
