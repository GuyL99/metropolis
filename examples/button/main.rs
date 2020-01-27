use metropolis::canvas::*;
use metropolis::color::*;
use metropolis::elements::*;
fn main(){
    let mut canv = Canvas::new(800,600);	
    let mut cnt = 0;
    canv.background(Color::from(20));
    let mut btn1 = button(110,110);
        let mut clicked = false;
    let draw = move |mut canvas:Canvas|->Canvas{	
      /*  if cnt%10==0{
            btn1.color(rgb(255,0,100));
        }
        if cnt%20==0{
            btn1.color(rgb(0,255,100));
        }*/
        if btn1.onHover(&canvas)&!clicked{
            btn1.color(rgb(255,100,00));
        }else{
            btn1.color(grayscale(255));
            if btn1.onClick(&canvas){
                clicked = true;
                btn1.color(rgb(255,0,100));
            }
        }
        canvas.attach(btn1);
        println!("{:?}",canvas.fps);
    //    cnt+=1;
        canvas
    };
    canv.show(draw);
}
