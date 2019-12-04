#[derive(Default, Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub tex_coords: [f32;2],
}
vulkano::impl_vertex!(Vertex, position, color,tex_coords);
#[derive(Default, Debug, Clone, Copy)]
pub struct Stext {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub text: &'static str,
    //pub font:Fonts,
}
/*pub enum Fonts{
    
}*/
