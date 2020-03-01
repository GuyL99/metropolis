use crate::color::*;
use crate::canvas::*;
pub trait PageElement {
    fn draw(self,canvas:&mut Canvas);
    #[allow(non_snake_case)]
    fn onClick(self,canvas:&Canvas)->bool;
    #[allow(non_snake_case)]
    fn onHover(self,canvas:&Canvas)->bool;
}
#[derive(Copy,Clone)]
pub enum Styles{
    Normal,
    RoundEdges,
    Triangular,
    Elliptic,
}
#[derive(Clone,Copy)]
pub struct Button{
    color:Color,
    x:u16,
    y:u16,
    width:u16,
    height:u16,
    border_width:u8,
    style:Styles,
    text:&'static str,
}
impl PageElement for Button {
    fn draw(self,canvas:&mut Canvas){
        canvas.textSize(12);
        canvas.text(self.x+self.border_width as u16+6,self.y+self.border_width as u16+(self.height/2)+1,self.text);
        match self.style{
            Styles::Normal=>{
                /*canvas.fill(self.color-20);
                canvas.rect(self.x,self.y,self.width,self.height);*/
                canvas.fill(self.color);
                canvas.rect(self.x+self.border_width as u16,self.y+self.border_width as u16,self.width-(self.border_width*2) as u16,self.height-(self.border_width*2) as u16);
            },
            Styles::RoundEdges=>{
                canvas.line(self.x+2,self.y,self.x+self.width-4,self.y);
                canvas.bezierCurveVertex((self.x+self.width-4) as i64,self.y as i64,(self.x+self.width-2) as i64,(self.y+2) as i64,(self.x+self.width-4) as i64,self.y as i64,(self.x+self.width-2) as i64,(self.y+2) as i64);
                canvas.line(self.x+self.width-2,self.y+2,self.x+self.width-2,self.y+self.height-4);
                canvas.bezierCurveVertex((self.x+self.width-2) as i64,(self.y+self.height-4) as i64,(self.x+self.width-4) as i64,(self.y+self.height-2) as i64,(self.x+self.width-2) as i64,(self.y+self.height-4) as i64,(self.x+self.width-4) as i64,(self.y+self.height-2) as i64);
                canvas.line(self.x+self.width-4,self.y+self.height-2,self.x+2,self.y+self.height-2);
                canvas.bezierCurveVertex((self.x+2) as i64,(self.y+self.height-2) as i64,(self.x) as i64,(self.y+self.height-4) as i64,(self.x+2) as i64,(self.y+self.height-2) as i64,(self.x) as i64,(self.y+self.height-4) as i64);
                canvas.line(self.x,self.y+self.height-4,self.x,self.y+2);
                canvas.bezierCurveVertex((self.x) as i64,(self.y+2) as i64,(self.x+2) as i64,self.y as i64,(self.x) as i64,(self.y+2) as i64,(self.x+2) as i64,self.y as i64);
            },
            _=>{},
        }
    }
    #[allow(non_snake_case)]
    fn onClick(self,canvas:&Canvas)->bool{
        if canvas.mouseClick()==MouseButton::Left && (canvas.mouseX()>self.x && canvas.mouseX()<self.x+self.width) &&(canvas.mouseY()>self.y && canvas.mouseY()<self.y+self.height){
            return true;
        }
        false
    }
    #[allow(non_snake_case)]
    fn onHover(self,canvas:&Canvas)->bool{
        if canvas.mouseClick()!=MouseButton::Left && (canvas.mouseX()>self.x && canvas.mouseX()<self.x+self.width) &&(canvas.mouseY()>self.y && canvas.mouseY()<self.y+self.height){
            return true;
        }
        false
    }
}
impl Button{
    pub fn new(x:u16,y:u16,text:&'static str)->Button{
        Button{color:Color::from(190), x,y,width:40,height:20,border_width:2,style:Styles::Normal,text,}
    }
    pub fn location(&mut self,x:u16,y:u16)->Self{
       self.x = x;
       self.y = y;
       *self
    }
    pub fn get_location(self)->(u16,u16){
        (self.x,self.y)
    }
    pub fn get_x(self)->u16{
        self.x
    }
    pub fn get_y(self)->u16{
        self.y
    }
    pub fn color(&mut self,color:Color)->Self{
        self.color = color;
        *self
    }
    pub fn size(&mut self,width:u16,height:u16)->Self{
        self.width = width;
        self.height= height;
        *self
    }
    pub fn get_size(self)->(u16,u16){
        (self.width,self.height)
    }
    pub fn get_width(self)->u16{
        self.width
    }
    pub fn get_height(self)->u16{
        self.height
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
    pub fn get_border_width(self)->u8{
        self.border_width
    }
    pub fn button_text(&mut self,text:&'static str)->Self{
        self.text = text;
        *self
    }
    pub fn get_color(self)->Color{
        self.color
    }
    pub fn style(&mut self,style:Styles)->Self{
        self.style = style;
        *self
    }
}
