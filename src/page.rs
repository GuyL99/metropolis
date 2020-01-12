use crate::elements::PageElements;
use crate::canvas::Canvas;
use crate::mapping;
use vulkano::image::Dimensions;
//use winit::MouseScrollDelta;
use winit::{ModifiersState/*,KeyboardInput,ElementState,Event, WindowEvent*/,VirtualKeyCode};
pub use winit::MouseButton;
use crate::color::Color;
#[derive(Copy,Clone,PartialEq)]
struct MouseScroll{
    pub delta:(i64,i64),
    pub moder:ModifiersState,
}
impl MouseScroll{
    pub fn new()->MouseScroll{
        let moder = ModifiersState{shift:false,ctrl:false,alt:false,logo:false};    
        let delta = (0,0);
        MouseScroll{delta,moder}
    }
    pub fn delta_x(self)->i64{
        self.delta.0
    }
    pub fn delta_y(self)->i64{
        self.delta.1//.PixelDelta.y as i64
    }
}
#[derive(Copy,Clone,PartialEq)]
struct Mouse{
    pub btn:Option<MouseButton>,
    pub moder:ModifiersState,
}
impl Mouse{
    pub fn new()->Mouse{
        let moder = ModifiersState{shift:false,ctrl:false,alt:false,logo:false};    
        let btn = None;
        Mouse{btn,moder}
    }
}
#[derive(Copy,Clone,PartialEq)]
struct Key{
    pub keycode:Option<VirtualKeyCode>,
    pub moder:ModifiersState,
    pub keep_key: bool,
}
impl Key{
    pub fn new()->Key{
        let moder = ModifiersState{shift:false,ctrl:false,alt:false,logo:false};    
        let keycode = None;
        Key{keycode,moder,keep_key:false}
    }
    pub fn get_mod(self)->ModifiersState{
        self.moder
    }
}
#[derive(Clone)]
struct Page{
    size: (u16, u16),
    background: [f32; 4],
    texture:Option<(Vec<u8>,Dimensions)>,
    elements:Vec<PageElements>,
    canvases:Vec<Canvas>,
    key:Key,
    cursor_pos:(u16,u16),
    mouse:Mouse,
    mouse_scroll:MouseScroll,
}
impl Page{
    ///returns the current key that is pressed on the mouse.
    #[allow(non_snake_case)]
    pub fn mouseClick(&mut self)->MouseButton{
        match self.mouse.btn{
        Some(btn)=> {return btn;},
        None=> {return MouseButton::Other(99);}
        }
    }
    ///returns the current key that is pressed.
    #[allow(non_snake_case)]
    pub fn keyPressed(&mut self)->VirtualKeyCode{
        match self.key.keycode{
        Some(key)=> {return key;},
        None=> {return VirtualKeyCode::Power;}
        }
    }
    ///keeps the key pressed in the key event until a new key is pressed
    #[allow(non_snake_case)]
    pub fn lockKeyEvent(&mut self){
         self.key.keep_key = true;
    }
    ///returns the x scroll delta of the mouse
    #[allow(non_snake_case)]
    pub fn mouseScrollX(&self)->i64{
        self.mouse_scroll.delta_x()
    }
    ///returns the y scroll delta of the mouse
    #[allow(non_snake_case)]
    pub fn mouseScrollY(&self)->i64{
        self.mouse_scroll.delta_y()
    }
    ///returns the x position of the mouse
    #[allow(non_snake_case)]
    pub fn mouseX(&self)->u16{
        self.cursor_pos.0
    }
    ///returns the y position of the mouse
    #[allow(non_snake_case)]
    pub fn mouseY(&self)->u16{
        self.cursor_pos.1
    }
    ///returns the current state of the modifiers
    pub fn get_modifiers(self)->ModifiersState{
        self.key.get_mod()
    }
    ///sets the background color(using the color struct).
    pub fn background(&mut self,color:Color){
        let r = color.get_r();
        let g = color.get_g();
        let b = color.get_b();
        let a = color.get_a();
        self.background = mapping::map_colors([r, g, b, a]);
    }
    ///creates a new page
    pub fn new(width:u16,height:u16)->Page{
        Page{
            size: (width, height),
            background:[0.0,0.0,0.0,1.0],
            texture:None,
            elements:vec![],
            canvases:vec![],
            key:Key::new(),
            cursor_pos:(0,0),
            mouse:Mouse::new(),
            mouse_scroll:MouseScroll::new(),
        }
    }
}
