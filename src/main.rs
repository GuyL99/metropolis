use vulkanoing::*;
fn main() {
    size(800,600);
    //rect(0,0,0,0);
    let mut cnt = 0;
    let draw =move || {
        /*let mut r=0;
        let g=0;
        let mut b=0;
        if cnt%3==0{
            r=255;
            b=100;
        }
        cnt+=1;*/
        //fill(255,0,100);
        rect(0,0,200,200);
        fill(255,0,100);
        rect(400,400,200,200);
        noFill();
    };
    show(draw);
}
