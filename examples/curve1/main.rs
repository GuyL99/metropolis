use metropolis::color::*;
use metropolis::*;
fn main() {
    let height = 900;
    let width = 1200;
    size(width, height);
    background(grayscale(100));
    let draw =move||{
    //let curve_vec:Vec<[i64;2]> = vec![[0,400],[3,370],[50,300],[75,257],[80,240],[150,150]];
    let curve_vec:Vec<[i64;2]> = vec![[400,0],[370,4],[300,50],[257,75],[80,240],[150,150]];
        curve(curve_vec);
    };
    show(draw);
}
