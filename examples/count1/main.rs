use metropolis::color::*;
use metropolis::*;
fn main() {
	size(800,600);
    background(grayscale(100));	
    textSize(45);
	let draw = move||{
		text(300,300,"text");
	};
	show(draw);
}
