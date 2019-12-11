use metropolis::canvas::*;
use metropolis::color::*;
fn main(){
	let mut canv = Canvas::new(800,600);	
    canv.background(grayscale(100));
    let mut posx = 0;
    let mut posy = 0;
	let draw = move |mut canvas:Canvas|->Canvas{	
		match canvas.keyPressed(){
		keyCode::W=>{posy-=10;},
		keyCode::A=>{posx-=10;},
		keyCode::S=>{posy+=10;},
		keyCode::D=>{posx+=10;},
		_=>{},
        }
        //println!("{:?}",canvas.mouseX());
        println!("{:?}",canvas.fps);
        canvas.fill(rgb(0,255,100));
        canvas.rect(posx,posy,20,20);
        canvas
	};
	canv.show(draw);
}
