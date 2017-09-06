#[derive(Copy, Clone,Debug)]
pub struct Vertex {
   pub position: (f32, f32, f32)
}
implement_vertex!(Vertex, position);