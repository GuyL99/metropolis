use crate::color::*;
use crate::canvas::*;
#[derive(Clone,Copy)]
pub struct Button{
    //canvas:Canvas,
    color:Color,
    x:u16,
    y:u16,
    width:u16,
    height:u16,
    border_width:u8,
}
#[derive(Clone)]
pub enum PageElements{
    Button(Button),
}
impl Button{
    pub fn new(canvas:Canvas,x:u16,y:u16)->Button{
        Button{/*canvas,*/color:Color::from(190), x,y,width:20,height:40,border_width:2,}
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
    /*
    pub fn draw(&mut self)->Canvas{
        self.canvas.fill(self.color-20);
        self.canvas.rect(self.x,self.y,self.height,self.width);
        self.canvas.fill(self.color);
        self.canvas.rect(self.x+self.border_width as u16,self.y+self.border_width as u16,self.height-(self.border_width*2) as u16,self.width-(self.border_width*2) as u16);
        self.canvas
    }
    #[allow(non_snake_case)]
    pub fn buttonClick(mut self)->bool{
        if self.canvas.mouseClick()==MouseButton::Left && (self.canvas.mouseX()>self.x && self.canvas.mouseX()<self.x+self.height) &&(self.canvas.mouseY()>self.y && self.canvas.mouseY()<self.y+self.width){
            return true;
        }
        false
    }*/
}
