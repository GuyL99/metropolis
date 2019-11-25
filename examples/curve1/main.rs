use metropolis::color::*;
use metropolis::*;
fn main() {
    let height = 600;
    let width = 800;
    size(width, height);
    background(grayscale(100));
    let draw = move || {
        let curve_vec: Vec<[i64; 2]> = vec![
            [0, 400],
            [30, 370],
            [50, 300],
            [75, 257],
            [80, 240],
            [150, 150],
            [250, 050],
        ];
        //let curve_vec:Vec<[i64;2]> = vec![[400,0],[370,4],[300,50],[257,75],[80,240],[150,150]];
        bezierCurve(curve_vec);
    };
    show(draw);
}
