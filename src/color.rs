#[derive(Copy,Clone,PartialEq,PartialOrd,Eq,Ord)]
pub struct Color{
    r:u8,
    g:u8,
    b:u8,
    a:u8
}
impl Color{
    pub fn get_g(self)->u8{
        self.g
    }
    pub fn get_r(self)->u8{
        self.r
    }
    pub fn get_b(self)->u8{
        self.b
    }
    pub fn get_a(self)->u8{
        self.a
    }
}
///retrun Color sruct from rgb values
pub fn rgb(r:u8,g:u8,b:u8)->Color{
    Color{r,g,b,a:255}
}
///retrun Color sruct from rgba values
pub fn rgba(r:u8,g:u8,b:u8,a:u8)->Color{
    Color{r,g,b,a}
}
///retrun Color sruct from grayscale values - need fixing
pub fn grayscale(gr:u8)->Color{
    Color{
        r:gr,
        g:gr,//(gr as f32 *0.59) as u8,
        b:gr,//(gr as f32 *0.11) as u8,
        a:255}
}
