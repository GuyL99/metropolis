use vulkanoing::*;
fn main() {
    let height = 900;
    let width = 1200;
    size(width,height);
    let mut spd = 0;
    let mut posy = 0;
    let draw =move || {
        spd+=1;
        if posy+50< height{
            posy+=spd;
        }
        //fill(255,0,100);
        //fill(0,0,00);
        //circle(400,posy,50);
        arc(500,500,100,180);
        //line(800,800,posy,posy);
    };
    show(draw);
}
