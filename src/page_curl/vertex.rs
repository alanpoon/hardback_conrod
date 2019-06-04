use crayon::impl_vertex;
use crayon::prelude::*;
impl_vertex! {
    Vertex {
        pos => [Position; Float; 3; false],
        uv =>[Texcoord0; Float; 2; false],
    }
}