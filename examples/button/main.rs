use metropolis::canvas::*;
use metropolis::color::*;
//use metropolis::elements::*;
fn main(){
    let mut canv = Canvas::new(800,600);	
    //canv.background(Color::from(220));
    let draw = |mut canvas:Canvas|->Canvas{	
        //canvas = canvas.button(400,400).draw();
        canvas
    };
    canv.show(draw);
}
