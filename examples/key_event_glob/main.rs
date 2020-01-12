use metropolis::color::*;
use metropolis::*;
fn main() {
	size(800,600);
    background(grayscale(100));
    textSize(45);
	let draw = move||{
    let mut str_from_count = "0";
		match keyPressed(){
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
		lockKeyEvent();
		text(300,300,str_from_count);
	};
	show(draw);
}
