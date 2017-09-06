#[derive(Copy, Clone,Debug)]
pub struct Vertex {
   pub position: (f32, f32, f32),
   pub tex_coords :(f32,f32)
}
implement_vertex!(Vertex, position,tex_coords);