use metropolis::color::*;
use metropolis::*;
use metropolis::math::*;
fn main() {
    let height = 600;
    let width = 800;
    size(width, height);
    background(grayscale(100));
        strokeWeight(8);
    let draw = move || {
        /*let curve_vec: Vec<[i64; 2]> = vec![
            [0, 400],
            [30, 370],
            [50, 300],
            [75, 257],
            [80, 240],
            [150, 150],
            [250, 050],
        ];*/
        //let curve_vec:Vec<[i64;2]> = vec![[400,0],[370,4],[300,50],[257,75],[80,240],[150,150]];
        //bezierCurve(curve_vec);
        //let arr1 = bezier_points(1,5,2,4,3,4,4,1);
        let arr1 = catmull_rom_chain(1,5,2,5,3,5,4,4);
        for pt in arr1.iter(){
            println!("val for x {} is: {}",pt[0],pt[1]);
        }
    };
    show(draw);
}
