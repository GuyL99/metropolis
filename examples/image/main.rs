use metropolis::color::*;
use metropolis::*;
fn main() {
    size(800,600);
    background(grayscale(100));
    let mut posy = 0;
    let mut cnt = 0;
    let image = img("/home/guyl/Desktop/rust.png");
    //let image = img("/home/guyl/Desktop/rust.png");
    let draw = move||{
        image.clone().display(0,00);//,1000,1000);
        if cnt%50 == 0{
            posy+=100;
        }
        cnt+=1;
        unsafe{
            println!("{}",FPS);
        }
	};
	show(draw);
}
