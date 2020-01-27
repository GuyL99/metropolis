use std::ops::Add;
use std::ops::Sub;
use std::fmt;
///a struct used for the coloring in this create
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
impl Color {
    pub fn get_g(self) -> u8 {
        self.g
    }
    pub fn get_r(self) -> u8 {
        self.r
    }
    pub fn get_b(self) -> u8 {
        self.b
    }
    pub fn get_a(self) -> u8 {
        self.a
    }
}
///retrun Color sruct from rgb values
pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color { r, g, b, a: 255 }
}
///retrun Color sruct from rgba values
pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color { r, g, b, a }
}
///retrun Color sruct from grayscale values - need fixing
pub fn grayscale(gr: u8) -> Color {
    Color {
        r: gr,
        g: gr, //(gr as f32 *0.59) as u8,
        b: gr, //(gr as f32 *0.11) as u8,
        a: 255,
    }
}
impl fmt::Display for Color{
    fn fmt(&self,f: &mut fmt::Formatter<'_>)->fmt::Result{
            write!(f, "({},{},{},{})", self.r, self.g,self.b,self.a)
    }
}
impl fmt::Debug for Color{
    fn fmt(&self,f: &mut fmt::Formatter<'_>)->fmt::Result{
            write!(f, "({},{},{},{})", self.r, self.g,self.b,self.a)
    }
}
impl From<(u8,u8,u8)> for Color{
    fn from(vals:(u8,u8,u8))->Self{
        Color{r:vals.0,g:vals.1,b:vals.2,a:255}
    }
}
impl From<(u8,u8,u8,u8)> for Color{
    fn from(vals:(u8,u8,u8,u8))->Self{
        Color{r:vals.0,g:vals.1,b:vals.2,a:vals.3}
    }
}
impl From<u8> for Color{
    fn from(vals:u8)->Self{
        Color{r:vals,g:vals,b:vals,a:255}
    }
}
impl Add<u8> for Color{
    type Output = Self;
    fn add(self,adder:u8)->Self{
        Color{r:self.get_r()+adder,g:self.get_g()+adder,b:self.get_b()+adder,a:self.get_a()}
    }
}
impl Sub<u8> for Color{
    type Output = Self;
    fn sub(self,adder:u8)->Self{
        let mut r:i16 = self.get_r() as i16 -adder as i16;
        let mut g:i16 = self.get_g() as i16 -adder as i16;
        let mut b:i16 = self.get_b() as i16 -adder as i16;
        if r<0{
            r=0;
        }
        if g<0{
            g=0;
        }
        if b<0{
            b=0;
        }
        Color{r:r as u8,g: g as u8,b: b as u8,a:self.get_a()}
    }
}
