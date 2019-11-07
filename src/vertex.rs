#[derive(Default, Debug, Clone,Copy)]
pub struct Vertex { 
    pub position: [f32; 2], 
    pub color: [f32;4],
}
vulkano::impl_vertex!(Vertex, position,color);
