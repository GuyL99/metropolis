use metropolis::color::*;
use metropolis::vector::*;
use metropolis::canvas::Canvas;
fn main(){
        let height = 600;
    let width = 800;
    //size(width, height);
    let mut canv:Canvas= Canvas::new(width,height);
    let mut vec1 = Vector{vec:vec![1,2,3,4,6,8,9,10]};
    vec1+=1;
    println!("{:?}",vec1);
    canv.background(grayscale(100));
    let draw = |mut canvas:Canvas|->Canvas {
        let curve_vec: Vec<[i64; 2]> = vec![
            [0, 400],
            [30, 370],
            [50, 300],
            [75, 257],
            [80, 240],
            [150, 150],
            [250, 050],
        ];
        canvas.bezierCurve(curve_vec);
        canvas
    };
    //canv = draw();
    canv.show(draw);

}
