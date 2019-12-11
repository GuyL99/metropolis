use metropolis::color::*;
use metropolis::math::*;
use metropolis::*;
fn main() {
	size(800,600);
	background(grayscale(100));
	let mut count = 0;
	let draw = move || {
		for i in 0..80{
			for j in 0..60{
				fill(grayscale(255));
				rect(i as u16*10u16,j as u16*10u16,10,10);
			}
		}
	count+=1;
	if count ==100{
	count =0;
}
	};
	show(draw);
}
