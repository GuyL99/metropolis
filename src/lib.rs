mod vertex;
use vertex::Vertex;
mod shaders;
mod setup;
mod mapping;
use mapping::*;
mod canvas;
use canvas::*;
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
pub fn size(wid:u16,hei:u16){
    unsafe{
        CANVAS.size = (wid,hei);
    }

}
pub fn show<F>(draw_fn:F)
    where F:FnMut()+ 'static{
    unsafe{
        CANVAS.show(draw_fn);
    }
}
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
pub fn circle(x:u16,y:u16,rad:u16){
    unsafe{
    let scale = [CANVAS.size.0,CANVAS.size.1];
    if CANVAS.stroke{
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
pub fn fill(r:u8,g:u8,b:u8){
    unsafe{
        CANVAS.fill = true;
        CANVAS.fill_color = mapping::map_colors([r,g,b,255]);
    }
}
pub fn stroke(r:u8,g:u8,b:u8){
    unsafe{
        CANVAS.stroke = true;
        CANVAS.color = mapping::map_colors([r,g,b,255]);
    }
}
pub fn background(r:u8,g:u8,b:u8){
    unsafe{
        CANVAS.background_color = mapping::map_colors([r,g,b,255]);
    }
}
#[allow(non_snake_case)]
pub fn strokeWeight(weight:u8){
    unsafe{
        CANVAS.stroke_weight = weight; 
    }
}
#[allow(non_snake_case)]
pub fn noFill(){
    unsafe{
        CANVAS.fill = false; 
    }
}
#[allow(non_snake_case)]
pub fn noStroke(){
    unsafe{
        CANVAS.stroke = false; 
    }
}
