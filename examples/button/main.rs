use metropolis::canvas::*;
use metropolis::color::*;
use metropolis::elements::*;
use metropolis::fonts::*;
fn main(){
    let mut canv = Canvas::new(800,600);	
    canv.background(Color::from(20));
    let mut btn1 = button(610,410,"btn1");//.style(Styles::RoundEdges);
    let mut clicked = false;
    let mut hovered = false;
    let draw = move |mut canvas:Canvas|->Canvas{
        canvas.font(Fonts::RobotoBlackItalic);
        canvas.text_color(rgb(255,0,0));
        //canvas.font(Fonts::DejaVuSans);
        if btn1.onHover(&canvas){
            if !clicked{
                btn1.color(rgb(255,100,00));
            }else{
                hovered = true;
                btn1.color(rgba(255,0,100,127));
            }
        }else{
            if btn1.onClick(&canvas){
                clicked = true;
                btn1.color(rgb(255,0,100));
            }else if !clicked{
                btn1.color(grayscale(255));
            }
        }
        if btn1.get_color()==rgba(255,0,100,127)&&!hovered{
            btn1.color(rgb(255,0,100));
        }
        //println!("{}",btn1.get_color());
        canvas.attach(btn1);
        //println!("{:?}",btn1.get_size());
        hovered = false;
        //println!("{:?}",canvas.fps);
        canvas
    };
    canv.show(draw);
}
