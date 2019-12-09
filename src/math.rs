use crate::compute::*;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
///converts degrees to radians
///
///```
///use metropolis::math::*;
///assert_eq!(rad(90.0),PI/2.0);
///```
pub fn rad(x: f32) -> f32 {
    (x / 180.0) * PI
}
///converts radians to degrees
///
///```
///use metropolis::math::*;
///assert_eq!(deg(PI),180.0);
///```
pub fn deg(x: f32) -> f32 {
    x * 180.0 / PI
}
/*
///uses Taylor's series to determine sinus at the x, for now accepts f32, in the future also u/i
pub fn sin(x: f32) -> f32 {
    #[allow(illegal_floating_point_literal_pattern)]
    match x {
        90.0 => {
            return 1.0;
        }
        0.0 => {
            return 0.0;
        }
        30.0 => {
            return 0.5;
        }
        _ => {
            let ang = rad(x % 360.0);
            return ang - ang.powf(3.0) / 6.0 + ang.powf(5.0) / 120.0 - ang.powf(7.0) / 5040.0;
        }
    };
}
/*pub fn cos<T>(x:T) -> f32
where T:Add+Sub+Div+Mul+Sub<T,Output=f32>+Mul<f32, Output =f32>+Copy+Div<f32, Output =f32>+Copy+Add<f32, Output =f32>+Copy+Sub<f32, Output =f32>+Sub<u64, Output =f32>+Div<u64, Output =u64>+Copy+Clone+PartialEq+Ord{
//where T:Add+Sub+Div+Mul+Sub<T, Output =f32>+Add<T, Output =f32>+Div<T, Output = f32>+Mul<T, Output = f32>+Mul<f32, Output =f32>+Copy+Div<f32, Output =f32>+Copy+Add<f32, Output =f32>+Copy+Sub<f32, Output =f32>+Sub<u64, Output =f32>+Div<u64, Output =u64>+Copy+Clone+PartialEq+PartialOrd{
//where T:numerous{*/
///uses sin(90-alpha) in order to calculate cosine
pub fn cos(x: f32) -> f32 {
    if x <= 90.0 {
        return sin(90.0 - x);
    } else if x <= 18.0 {
        return -1.0 * sin(180.0 - x);
    } else if x <= 270.0 {
        return sin(270.0 - x);
    } else {
        return -1.0 * sin(360.0 - x);
    }
}
///uses sin(x)/cos(x) in order to calculate tan(x)
pub fn tan(x: f32) -> f32 {
    sin(x) / cos(x)
}*/
///calculates the factorial of a number
///
///```
///use metropolis::math::*;
///assert_eq!(factorial(6),720);
///```
pub fn factorial(n: u64) -> u64 {
    real_factorial(n, 1)
}
fn real_factorial(n: u64, mut accume: u64) -> u64 {
    if n == 1 {
        return accume;
    }
    accume *= n;
    real_factorial(n - 1, accume)
}
/*///returns the absolute value of the number i number
///
///```
///use metropolis::math::*;
///assert_eq!(abs(-1),1);
///```
pub fn abs<T>(x: T) -> T
where
    T: Copy + Clone + Add + Sub + Div + Mul+ Mul<i8, Output = T> + PartialOrd+PartialEq,
{
    if x<(x*-1){
        return x*-1;
    }
    x
}
///returns the absolute value of the number f number
///
///```
///use metropolis::math::*;
///assert_eq!(absf(-1.5),1.5);
///```
pub fn absf<T>(x: T) -> T
where
    T: Copy + Clone + Add + Sub + Div + Mul+ Mul<f32, Output = T> + PartialOrd+PartialEq,
{
    if x<(x*-1.0){
        return x*-1.0;
    }
    x
}*/
///converts from one range to another
///
///```
///use metropolis::math::*;
///assert_eq!(map(1.0,0.0,2.0,0.0,1.0),0.5);
///```
pub fn map<T>(x: T, a1: T, a2: T, b1: T, b2: T) -> f64
where
    T: Copy + Clone + Add + Sub + Div + Mul + Sub<T, Output = f64> + Add<f64, Output = f64>,
{
    b1 + ((x - a1) / (a2 - a1) * (b2 - b1))
}
#[allow(clippy::approx_constant)]
pub const PI: f32 = 3.14159265359;
pub const TWO_PI: f32 = PI * 2.0;
///create evenly spaced points inside the space between two points:
///```
///use metropolis::math::*;
///assert_eq!(linspace(0.0,1.0,4),vec![0.0,0.25,0.5,0.75,1.0]);
///```
pub fn linspace(start: f64, finish: f64, n: u64) -> Vec<f64> {
    let mut vec1 = vec![];
    for i in 0..(n + 1) {
        vec1.push(((finish - start) / n as f64) * i as f64 + start);
    }
    vec1
}
///takes 8 values(4 x's and 4 y's ad constructs a 100 points array of a catmull rom chain curve
///from them using he algorithm.
pub fn catmull_rom_chain(
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
    x3: i64,
    y3: i64,
    x4: i64,
    y4: i64,
) -> [[f64; 2]; 100] {
    let t0 = 0f64;
    let t1 = (((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f64).sqrt().sqrt() + t0 as f64;
    let t2 = (((x3 - x2).pow(2) + (y3 - y2).pow(2)) as f64).sqrt().sqrt() + t1 as f64;
    let t3 = (((x4 - x3).pow(2) + (y4 - y3).pow(2)) as f64).sqrt().sqrt() + t2 as f64;
    let t = linspace_in(t1, t2);
    let a11 = [
        compute_ops(
            compute_ops(
                compute_ops(compute_ops(t, -1.0, ops::FloatMult), t1, ops::FloatAdd),
                t1 - t0,
                ops::FloatDiv,
            ),
            x1 as f64,
            ops::FloatMult,
        ),
        compute_ops(
            compute_ops(
                compute_ops(compute_ops(t, -1f64, ops::FloatMult), t1, ops::FloatAdd),
                t1 - t0,
                ops::FloatDiv,
            ),
            y1 as f64,
            ops::FloatMult,
        ),
    ];
    let a12 = [
        compute_ops(
            compute_ops(compute_ops(t, t0, ops::FloatSub), t1 - t0, ops::FloatDiv),
            x2 as f64,
            ops::FloatMult,
        ),
        compute_ops(
            compute_ops(compute_ops(t, t0, ops::FloatAdd), t1 - t0, ops::FloatDiv),
            y2 as f64,
            ops::FloatMult,
        ),
    ];
    let a1 = [
        compute_ops2(a11[0], a12[0], ops::FloatAddVecs),
        compute_ops2(a11[1], a12[1], ops::FloatAddVecs),
    ];
    let a21 = [
        compute_ops(
            compute_ops(
                compute_ops(compute_ops(t, -1.0, ops::FloatMult), t2, ops::FloatAdd),
                t2 - t1,
                ops::FloatDiv,
            ),
            x2 as f64,
            ops::FloatMult,
        ),
        compute_ops(
            compute_ops(
                compute_ops(compute_ops(t, -1f64, ops::FloatMult), t2, ops::FloatAdd),
                t2 - t1,
                ops::FloatDiv,
            ),
            y2 as f64,
            ops::FloatMult,
        ),
    ];
    let a22 = [
        compute_ops(
            compute_ops(compute_ops(t, t1, ops::FloatSub), t2 - t1, ops::FloatDiv),
            x3 as f64,
            ops::FloatMult,
        ),
        compute_ops(
            compute_ops(compute_ops(t, t1, ops::FloatAdd), t2 - t1, ops::FloatDiv),
            y3 as f64,
            ops::FloatMult,
        ),
    ];
    let a2 = [
        compute_ops2(a21[0], a22[0], ops::FloatAddVecs),
        compute_ops2(a21[1], a22[1], ops::FloatAddVecs),
    ];
    let a31 = [
        compute_ops(
            compute_ops(
                compute_ops(compute_ops(t, -1.0, ops::FloatMult), t3, ops::FloatAdd),
                t3 - t2,
                ops::FloatDiv,
            ),
            x3 as f64,
            ops::FloatMult,
        ),
        compute_ops(
            compute_ops(
                compute_ops(compute_ops(t, -1f64, ops::FloatMult), t3, ops::FloatAdd),
                t3 - t2,
                ops::FloatDiv,
            ),
            y3 as f64,
            ops::FloatMult,
        ),
    ];
    let a32 = [
        compute_ops(
            compute_ops(compute_ops(t, t2, ops::FloatSub), t3 - t2, ops::FloatDiv),
            x4 as f64,
            ops::FloatMult,
        ),
        compute_ops(
            compute_ops(compute_ops(t, t2, ops::FloatAdd), t3 - t2, ops::FloatDiv),
            y4 as f64,
            ops::FloatMult,
        ),
    ];
    let a3 = [
        compute_ops2(a31[0], a32[0], ops::FloatAddVecs),
        compute_ops2(a31[1], a32[1], ops::FloatAddVecs),
    ];
    let b11 = [
        compute_ops2(
            compute_ops(
                compute_ops(compute_ops(t, -1.0, ops::FloatMult), t2, ops::FloatAdd),
                t2 - t0,
                ops::FloatDiv,
            ),
            a1[0],
            ops::FloatMultVecs,
        ),
        compute_ops2(
            compute_ops(
                compute_ops(compute_ops(t, -1f64, ops::FloatMult), t2, ops::FloatAdd),
                t2 - t0,
                ops::FloatDiv,
            ),
            a1[1],
            ops::FloatMultVecs,
        ),
    ];
    let b12 = [
        compute_ops2(
            compute_ops(compute_ops(t, t0, ops::FloatSub), t2 - t0, ops::FloatDiv),
            a2[0],
            ops::FloatMultVecs,
        ),
        compute_ops2(
            compute_ops(compute_ops(t, t2, ops::FloatAdd), t2 - t0, ops::FloatDiv),
            a2[1],
            ops::FloatMultVecs,
        ),
    ];
    let b1 = [
        compute_ops2(b11[0], b12[0], ops::FloatAddVecs),
        compute_ops2(b11[1], b12[1], ops::FloatAddVecs),
    ];
    let b21 = [
        compute_ops2(
            compute_ops(
                compute_ops(compute_ops(t, -1.0, ops::FloatMult), t3, ops::FloatAdd),
                t3 - t1,
                ops::FloatDiv,
            ),
            a2[0],
            ops::FloatMultVecs,
        ),
        compute_ops2(
            compute_ops(
                compute_ops(compute_ops(t, -1f64, ops::FloatMult), t3, ops::FloatAdd),
                t3 - t1,
                ops::FloatDiv,
            ),
            a2[1],
            ops::FloatMultVecs,
        ),
    ];
    let b22 = [
        compute_ops2(
            compute_ops(compute_ops(t, t1, ops::FloatSub), t3 - t1, ops::FloatDiv),
            a3[0],
            ops::FloatMultVecs,
        ),
        compute_ops2(
            compute_ops(compute_ops(t, t1, ops::FloatAdd), t3 - t1, ops::FloatDiv),
            a3[1],
            ops::FloatMultVecs,
        ),
    ];
    let b2 = [
        compute_ops2(b21[0], b22[0], ops::FloatAddVecs),
        compute_ops2(b21[1], b22[1], ops::FloatAddVecs),
    ];

    let c11 = [
        compute_ops2(
            compute_ops(
                compute_ops(compute_ops(t, -1.0, ops::FloatMult), t2, ops::FloatAdd),
                t2 - t1,
                ops::FloatDiv,
            ),
            b1[0],
            ops::FloatMultVecs,
        ),
        compute_ops2(
            compute_ops(
                compute_ops(compute_ops(t, -1f64, ops::FloatMult), t2, ops::FloatAdd),
                t2 - t1,
                ops::FloatDiv,
            ),
            b1[1],
            ops::FloatMultVecs,
        ),
    ];
    let c12 = [
        compute_ops2(
            compute_ops(compute_ops(t, t1, ops::FloatSub), t2 - t1, ops::FloatDiv),
            b2[0],
            ops::FloatMultVecs,
        ),
        compute_ops2(
            compute_ops(compute_ops(t, t1, ops::FloatAdd), t2 - t1, ops::FloatDiv),
            b2[1],
            ops::FloatMultVecs,
        ),
    ];
    let c1 = [
        compute_ops2(c11[0], c12[0], ops::FloatAddVecs),
        compute_ops2(c11[1], c12[1], ops::FloatAddVecs),
    ];
    let mut c: [[f64; 2]; 100] = [[0f64; 2]; 100];
    for i in 0..99 {
        c[i] = [c1[0][i], c1[1][i]];
    }
    c
}
fn linspace_in(start: f64, finish: f64) -> [f64; 100] {
    let mut arr1: [f64; 100] = [0.0; 100];
    for i in 0..(99) {
        arr1[i] = ((finish - start) / 100 as f64) * i as f64 + start;
    }
    arr1
}
pub fn bezier_points(
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
    x3: i64,
    y3: i64,
    x4: i64,
    y4: i64,
) -> [[f64; 2]; 101] {
    let mut t: f64;
    let mut b: [f64; 2];
    let mut arr_b = [[0f64; 2]; 101];
    for i in 0..101 {
        t = i as f64 / 100.0;
        b = [
            ((1.0 - t).powf(3.0) * x1 as f64)
                + (3.0 * (1.0 - t).powf(2.0) * t) * x2 as f64
                + (3.0 * (1.0 - t) * t.powf(2.0)) * x3 as f64
                + (t.powf(3.0)) * x4 as f64,
            ((1.0 - t).powf(3.0) * y1 as f64)
                + (3.0 * (1.0 - t).powf(2.0) * t) * y2 as f64
                + (3.0 * (1.0 - t) * t.powf(2.0)) * y3 as f64
                + (t.powf(3.0)) * y4 as f64,
        ];
        arr_b[i] = b;
    }
    arr_b
}
