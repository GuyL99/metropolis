use metropolis::color::*;
use metropolis::canvas::*;
fn main() {
    let mut canv = Canvas::new(800,600);
    //canv.background(grayscale(100));
    let mut posy = 0;
    let mut cnt = 0;
    let mut r = 100;
    let mut g = 100;
    let mut b = 100;
    //let image = img("/home/guyl/Desktop/saitama.png");
    let image = img("/home/guyl/Desktop/rust.png");//.dimensions(500,500);
    let draw = move |mut canvas:Canvas|->Canvas{
        canvas.display(image.clone(),canvas.mouseX(),canvas.mouseY());
        //canvas.textSize(128);
        //canvas.text(100,100,"ttsstttadsas");
        //canvas.text(500,500,"tttttadsas");
        /*println!("{}",Color::from((r,g,b)));
        canvas.background(rgb(r,g,b));
        if cnt%50 == 0{
            posy+=100;
        }
        cnt+=1;
        if r == 255{
            r=0;
            g=0;
            b=0;
        }
        if g == 255{
            r+=1;
            g=0;
            b=0;
        }
        if b == 255{
            g+=1;
            b=0;
        }
        b+=1;
        canvas.fill(Color::from((0,0,0)));
        canvas.rect(0,0,20,20);
        canvas.fill(Color::from((100,160,200)));
        canvas.rect(30,30,20,20);*/
        println!("{}",canvas.fps);
        canvas
	};
	canv.show(draw);
}
