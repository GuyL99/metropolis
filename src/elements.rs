use crate::color::*;
use crate::canvas::*;
pub trait PageElement {
    fn draw(self,canvas:&mut Canvas);
    #[allow(non_snake_case)]
    fn onClick(self,canvas:&Canvas)->bool;
    #[allow(non_snake_case)]
    fn onHover(self,canvas:&Canvas)->bool;
}
#[derive(Clone,Copy)]
pub struct Button{
    color:Color,
    x:u16,
    y:u16,
    width:u16,
    height:u16,
    border_width:u8,
}
impl PageElement for Button {
    fn draw(self,canvas:&mut Canvas){
        canvas.fill(self.color-20);
        canvas.rect(self.x,self.y,self.height,self.width);
        canvas.fill(self.color);
        canvas.rect(self.x+self.border_width as u16,self.y+self.border_width as u16,self.height-(self.border_width*2) as u16,self.width-(self.border_width*2) as u16);
    }
    #[allow(non_snake_case)]
    fn onClick(self,canvas:&Canvas)->bool{
        if canvas.mouseClick()==MouseButton::Left && (canvas.mouseX()>self.x && canvas.mouseX()<self.x+self.height) &&(canvas.mouseY()>self.y && canvas.mouseY()<self.y+self.width){
            return true;
        }
        false
    }
    #[allow(non_snake_case)]
    fn onHover(self,canvas:&Canvas)->bool{
        if canvas.mouseClick()!=MouseButton::Left && (canvas.mouseX()>self.x && canvas.mouseX()<self.x+self.height) &&(canvas.mouseY()>self.y && canvas.mouseY()<self.y+self.width){
            return true;
        }
        false
    }
}
/*
#[derive(Clone)]
pub enum PageElements{
    Button(Button),
}*/
impl Button{
    pub fn new(x:u16,y:u16)->Button{
        Button{color:Color::from(190), x,y,width:20,height:40,border_width:2,}
    }
    pub fn location(&mut self,x:u16,y:u16){
       self.x = x;
       self.y = y;
    }
    pub fn color(&mut self,color:Color){
        self.color = color;
    }
    pub fn size(&mut self,width:u16,height:u16)->Self{
        self.width = width;
        self.height= height;
        *self
    }
    pub fn width(&mut self,width:u16)->Self{
        self.width = width;
        *self
    }
    pub fn height(&mut self,height:u16)->Self{
        self.height = height;
        *self
    }
    
    pub fn border_width(&mut self,width:u8)->Self{
        self.border_width = width;
        *self
    }
}
