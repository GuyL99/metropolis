use metropolis::color::*;
use metropolis::*;
fn main() {
	size(800,600);
    background(grayscale(100));
	let mut count = 0;	
let mut frames = 0;
    let mut str_from_count = "0";
	let mut state =1;
    textSize(45);
	let draw = move||{
		unsafe{
			println!("{}",FPS);
		}
		frames += 1;

		match count{
		0=>{str_from_count = "00";},
		1=>{str_from_count = "10";},
		2=>{str_from_count = "02";},
		3=>{str_from_count = "3";},
		4=>{str_from_count = "4";},
		5=>{str_from_count = "5";},
		6=>{str_from_count = "6";},
		7=>{str_from_count = "7";},
		8=>{str_from_count = "8";},
		9=>{str_from_count = "9";},
		_=>{},
}
		text(300,300,str_from_count);
	if frames % 15 == 0{
	count += state; 
}
	if count == 9||count==0	{
	state *= -1
}
	};
	show(draw);
}
