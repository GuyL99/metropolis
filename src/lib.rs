//!this crate is a high level easy to use graphics renderer inspired by processing in java and p5 in
//!javascript. Working with it utilizes high level function like arc,line,rect and such that are
//!used to draw things directly on a fixed canvas(there will later be added a module that allows
//!you to make sveral canvases and display them as you wish. 3D is also coming and is currently
//!under development(for now it's just 2D functions).
//!the way to use the library is to use the size function to create a canvas with a fixed
//!size(width,height), afterwards, you create some setup variable and setup the background for the
//!animation/game/test/simulation you want to run, then you create a closure and save it to a variable, 
//!and finally send it to the show function(designed to loop over the draw function).
//!like this (grvaity example):
//!```
//!use metro::*;
//!use metro::color::*;
//!fn main() {
//!   //here I set up the background, height, width,spd,and the position on y
//!   let height = 900;
//!   let width = 1200;
//!   size(width,height);
//!   let mut spd = 0;
//!   let mut posy = 0;
//!   background(grayscale(100));
//!   let draw =move || {
//!   //inside the draw function I create the accelaration to simulate gravity
//!       spd+=1;
//!      if posy+50< height{
//!           posy+=spd;
//!       }
//!       // and those are the library functions fill-which makes the filled color be pinkish
//!       // and circle which draws a circle with a center in 400(in x) and posy(changing y), with a
//!       //radius of 100.
//!   fill(rgb(255,0,100));
//!   circle(400,posy,100);
//!  };
//!  //finally I send the draw function into show like that
//!   show(draw);
//!}
//!```
mod vertex;
use vertex::Vertex;
mod shaders;
mod setup;
mod mapping;
use mapping::*;
mod canvas;
use canvas::*;
///a module to provide some mathematical help functions from the crate.
///Will be much expanded upon in the near future.
pub mod math;
///a module used for coloring in this crate, will be adding more functions and easier set in the
///future.
pub mod color;
use color::*;
fn add_to_fill(pusher:Vertex){
    unsafe{
    match &FILL_VERTECIES{
        None=>{FILL_VERTECIES = Some(vec![pusher]);},
        Some(vec1)=>{let mut vec2 = vec1.clone();
            vec2.push(pusher);
            FILL_VERTECIES = Some(vec2);}
    };
    }
}
fn add_to_stroke(pusher:Vertex){
    unsafe{
    match &STROKE_VERTECIES{
        None=>{STROKE_VERTECIES = Some(vec![pusher]);},
        Some(vec1)=>{let mut vec2 = vec1.clone();
            vec2.push(pusher);
            STROKE_VERTECIES = Some(vec2);}
    };
    }
}
///creates the canvas with the width and height sent to this function
pub fn size(width:u16,height:u16){
    unsafe{
        CANVAS.size = (width,height);
    }

}
///this is the function used to run the animation
pub fn show<F>(draw_fn:F)
    where F:FnMut()+ 'static{
    unsafe{
        CANVAS.show(draw_fn);
    }
}
///recieves the x and y of the top spot and then the width and height of the rectangle you want
///built.
pub fn rect(x:u16,y:u16,width:u16,height:u16){
    unsafe{
        let scale = [CANVAS.size.0,CANVAS.size.1];
        let t_l = map([x,y],scale);
        let b_r = map([x+width,y+height],scale);
        let t_r = map([x+width,y],scale);
        let b_l = map([x,y+height],scale);
        if CANVAS.fill{
            let color = CANVAS.fill_color;
            add_to_fill(Vertex{ position: b_r ,color});
            add_to_fill(Vertex{ position: t_r ,color});
            add_to_fill(Vertex{ position: t_l ,color});
            add_to_fill(Vertex{ position: t_l ,color});
            add_to_fill(Vertex{ position: b_l ,color});
            add_to_fill(Vertex{ position: b_r ,color});
        }
        if CANVAS.stroke{
            let color = CANVAS.color;
            add_to_stroke(Vertex{ position: t_l ,color});
            add_to_stroke(Vertex{ position: t_r ,color});
            add_to_stroke(Vertex{ position: t_r ,color});
            add_to_stroke(Vertex{ position: b_r ,color});
            add_to_stroke(Vertex{ position: b_r ,color});
            add_to_stroke(Vertex{ position: b_l ,color});
            add_to_stroke(Vertex{ position: b_l ,color});
            add_to_stroke(Vertex{ position: t_l ,color});
        }
    }
}
///recieves the x and y of the top point and then the x and the y of the bottom point and creates a
///line between them.
pub fn line(x:u16,y:u16,x2:u16,y2:u16){
    unsafe{
        let scale = [CANVAS.size.0,CANVAS.size.1];
        let srt = map([x,y],scale);
        let fin = map([x2,y2],scale);
        let color = CANVAS.color;
        add_to_stroke(Vertex{ position: srt ,color});
        add_to_stroke(Vertex{ position: fin ,color});
    }
}
///recieves the x and y of the 3 points of the triangle and creates it based on them
pub fn triangle(_pt1:(u16,u16),_pt2:(u16,u16),_pt3:(u16,u16)){
    unsafe{
        let scale = [CANVAS.size.0,CANVAS.size.1];
        let pt1 = map([_pt1.0,_pt1.1],scale);
        let pt2 = map([_pt2.0,_pt2.1],scale);
        let pt3 = map([_pt3.0,_pt3.1],scale);
        if CANVAS.fill{
            let color = CANVAS.fill_color;
            add_to_fill(Vertex{ position: pt1 ,color});
            add_to_fill(Vertex{ position: pt2 ,color});
            add_to_fill(Vertex{ position: pt3 ,color});
        }
        if CANVAS.stroke{
            let color = CANVAS.color;
            add_to_stroke(Vertex{ position: pt1 ,color});
            add_to_stroke(Vertex{ position: pt2 ,color});
            add_to_stroke(Vertex{ position: pt2 ,color});
            add_to_stroke(Vertex{ position: pt3 ,color});
            add_to_stroke(Vertex{ position: pt3 ,color});
            add_to_stroke(Vertex{ position: pt1 ,color});
        }
    }
}
///recieves the x and the y of the center of the ellipse and the width and height of the ellipse
///and creates it accordingly
pub fn ellipse(x:u16,y:u16,a:u16,b:u16){
    unsafe{
    let scale = [CANVAS.size.0,CANVAS.size.1];
    if CANVAS.stroke && !(CANVAS.fill &&CANVAS.color == CANVAS.fill_color){
        let mut pt_x = x as f32 + a as f32;
        let mut pt_y = y as f32;
        for an in (0..360).step_by(6){
            let ptx = x as f32+((an as f32/360.0)*6.28).cos()*a as f32;
            let pty = y as f32+((an as f32/360.0)*6.28).sin()*b as f32;
            add_to_stroke(Vertex{ position:map_circ([pt_x,pt_y],scale),color:CANVAS.color});
            add_to_stroke(Vertex{ position:map_circ([ptx,pty],scale),color:CANVAS.color});
            pt_x =ptx;
            pt_y = pty;
        }
        add_to_stroke(Vertex{ position:map_circ([pt_x,pt_y],scale),color:CANVAS.color});
        pt_x = x as f32+a as f32+0.5;
        pt_y = y as f32+0.5;
        add_to_stroke(Vertex{ position:map_circ([pt_x,pt_y],scale),color:CANVAS.color});
    }
    if CANVAS.fill{
        let mut pt_x = x as f32 + a as f32;
        let mut pt_y = y as f32;
        for an in (0..360).step_by(6){
            let ptx = x as f32+((an as f32/360.0)*6.28).cos()*a as f32;
            let pty = y as f32+((an as f32/360.0)*6.28).sin()*b as f32;
            add_to_fill(Vertex{ position:map_circ([pt_x,pt_y],scale),color:CANVAS.fill_color});
            add_to_fill(Vertex{ position:map_circ([ptx,pty],scale),color:CANVAS.fill_color});
            add_to_fill(Vertex{ position:map_circ([x as f32,y as f32],scale),color:CANVAS.fill_color});
            pt_x =ptx;
            pt_y = pty;
        }
        add_to_fill(Vertex{ position:map_circ([pt_x,pt_y],scale),color:CANVAS.fill_color});
        pt_x = x as f32+a as f32+0.5;
        pt_y = y as f32+0.5;
        add_to_fill(Vertex{ position:map_circ([pt_x,pt_y],scale),color:CANVAS.fill_color});
        add_to_fill(Vertex{ position:map_circ([x as f32,y as f32],scale),color:CANVAS.fill_color});
    }
    }
}
///recieves the x and y of the center of the circle and the radius and builds it with them.
pub fn circle(x:u16,y:u16,rad:u16){
    unsafe{
    let scale = [CANVAS.size.0,CANVAS.size.1];
    if CANVAS.stroke && !(CANVAS.fill &&CANVAS.color == CANVAS.fill_color){
        let mut pt_x = x as f32 + rad as f32;
        let mut pt_y = y as f32;
        for a in (0..360).step_by(6){
            let ptx = x as f32+((a as f32/360.0)*6.28).cos()*rad as f32;
            let pty = y as f32+((a as f32/360.0)*6.28).sin()*rad as f32;
            add_to_stroke(Vertex{ position:map_circ([pt_x,pt_y],scale),color:CANVAS.color});
            add_to_stroke(Vertex{ position:map_circ([ptx,pty],scale),color:CANVAS.color});
            pt_x =ptx;
            pt_y = pty;
        }
        add_to_stroke(Vertex{ position:map_circ([pt_x,pt_y],scale),color:CANVAS.color});
        pt_x = x as f32+rad as f32+0.5;
        pt_y = y as f32+0.5;
        add_to_stroke(Vertex{ position:map_circ([pt_x,pt_y],scale),color:CANVAS.color});
    }
    if CANVAS.fill{
        let mut pt_x = x as f32 + rad as f32;
        let mut pt_y = y as f32;
        for a in (0..360).step_by(6){
            let ptx = x as f32+((a as f32/360.0)*6.28).cos()*rad as f32;
            let pty = y as f32+((a as f32/360.0)*6.28).sin()*rad as f32;
            add_to_fill(Vertex{ position:map_circ([pt_x,pt_y],scale),color:CANVAS.fill_color});
            add_to_fill(Vertex{ position:map_circ([ptx,pty],scale),color:CANVAS.fill_color});
            add_to_fill(Vertex{ position:map_circ([x as f32,y as f32],scale),color:CANVAS.fill_color});
            pt_x =ptx;
            pt_y = pty;
        }
        add_to_fill(Vertex{ position:map_circ([pt_x,pt_y],scale),color:CANVAS.fill_color});
        pt_x = x as f32+rad as f32+0.5;
        pt_y = y as f32+0.5;
        add_to_fill(Vertex{ position:map_circ([pt_x,pt_y],scale),color:CANVAS.fill_color});
        add_to_fill(Vertex{ position:map_circ([x as f32,y as f32],scale),color:CANVAS.fill_color});
    }
    }
}
///recieves the x and the y and makes a small circle in the spot(size depends on strokeWeight).
pub fn point(x:u16,y:u16){
    unsafe{
    let stro =CANVAS.stroke; 
    let fil =CANVAS.fill; 
    CANVAS.stroke = false;
    CANVAS.fill = true;
    circle(x,y,CANVAS.stroke_weight as u16);
    CANVAS.stroke = stro;
    CANVAS.fill = fil;
    }
}
///enables fill and receives the color of the fill(the struct color) and sets the fill color to be
///the color.
pub fn fill(color:Color){
    let r = color.get_r();
    let g = color.get_g();
    let b = color.get_b();
    let a = color.get_a();
    unsafe{
        CANVAS.fill = true;
        CANVAS.fill_color = mapping::map_colors([r,g,b,a]);
    }
}
///enables stroke and receives the color of the stroke(the struct color) and sets the stroke color to be
///the color.
pub fn stroke(color:Color){
    let r = color.get_r();
    let g = color.get_g();
    let b = color.get_b();
    let a = color.get_a();
    unsafe{
        CANVAS.stroke = true;
        CANVAS.color = mapping::map_colors([r,g,b,a]);
    }
}
///sets the background color(using the color struct).
pub fn background(color:Color){
    let r = color.get_r();
    let g = color.get_g();
    let b = color.get_b();
    let a = color.get_a();
    unsafe{
        CANVAS.background_color = mapping::map_colors([r,g,b,a]);
    }
}
///sets the stroke weight(the width of lines and points
#[allow(non_snake_case)]
pub fn strokeWeight(weight:u8){
    unsafe{
        CANVAS.stroke_weight = weight; 
    }
}
///disables fill on the canvas.
#[allow(non_snake_case)]
pub fn noFill(){
    unsafe{
        CANVAS.fill = false; 
    }
}
///disables stroke on the canvas.
#[allow(non_snake_case)]
pub fn noStroke(){
    unsafe{
        CANVAS.stroke = false; 
    }
}
///create an arc from a circle, recieves the center of the circle and the radius and the degrees
///covered by the arc (360 degree arc is a full circle).
pub fn arc(x:u16,y:u16,rad:u16,deg:u16){
    unsafe{
    let scale = [CANVAS.size.0,CANVAS.size.1];
    if CANVAS.stroke && !(CANVAS.fill &&CANVAS.color == CANVAS.fill_color){
        let mut pt_x = x as f32 + rad as f32;
        let mut pt_y = y as f32;
        for a in (0..deg+6).step_by(6){
            let ptx = x as f32+((a as f32/360.0)*6.28).cos()*rad as f32;
            let pty = y as f32+((a as f32/360.0)*6.28).sin()*rad as f32;
            add_to_stroke(Vertex{ position:map_circ([pt_x,pt_y],scale),color:CANVAS.color});
            add_to_stroke(Vertex{ position:map_circ([ptx,pty],scale),color:CANVAS.color});
            pt_x =ptx;
            pt_y = pty;
        }
    }
    if CANVAS.fill{
        let mut pt_x = x as f32 + rad as f32;
        let mut pt_y = y as f32;
        for a in (0..deg+6).step_by(6){
            let ptx = x as f32+((a as f32/360.0)*6.28).cos()*rad as f32;
            let pty = y as f32+((a as f32/360.0)*6.28).sin()*rad as f32;
            add_to_fill(Vertex{ position:map_circ([pt_x,pt_y],scale),color:CANVAS.fill_color});
            add_to_fill(Vertex{ position:map_circ([ptx,pty],scale),color:CANVAS.fill_color});
            add_to_fill(Vertex{ position:map_circ([x as f32,y as f32],scale),color:CANVAS.fill_color});
            pt_x =ptx;
            pt_y = pty;
        }
    }
    }
}
