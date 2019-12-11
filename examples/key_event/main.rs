use metropolis::canvas::*;
use metropolis::color::*;
fn main(){
	let mut canv = Canvas::new(800,600);	
    canv.background(grayscale(100));
    canv.textSize(45);
	let draw = |mut canvas:Canvas|->Canvas{	
        let mut str_from_count = "0";
		match canvas.keyPressed(){
		keyCode::Key0=>{str_from_count = "0";},
		keyCode::Key1=>{str_from_count = "1";},
		keyCode::Key2=>{str_from_count = "2";},
		keyCode::Key3=>{str_from_count = "3";},
		keyCode::Key4=>{str_from_count = "4";},
		keyCode::Key5=>{str_from_count = "5";},
		keyCode::Key6=>{str_from_count = "6";},
		keyCode::Key7=>{str_from_count = "7";},
		keyCode::Key8=>{str_from_count = "8";},
		keyCode::Key9=>{str_from_count = "9";},
		_=>{},
}
        println!("{:?}",canvas.mouseX());
		canvas.text(300,300,str_from_count);
        canvas
	};
	canv.show(draw);
}
