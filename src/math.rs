/*use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;
use std::ops::Mul;
trait numerous{
}
impl numerous for f32 {}
impl numerous for f64 {}
impl numerous for u8  {}
impl numerous for u16 {}
impl numerous for u32 {}
impl numerous for u64 {}
impl numerous for i8  {}
impl numerous for i16 {}
impl numerous for i32 {}
impl numerous for i64 {}*/
/*pub fn sin<T>(x:T) -> f32
where T:Add+Sub+Div+Mul+Sub<T,Output=f32>+Mul<f32, Output =f32>+Copy+Div<f32, Output =f32>+Copy+Add<f32, Output =f32>+Copy+Sub<f32, Output =f32>+Sub<u64, Output =f32>+Div<u64, Output =u64>+Copy+Clone+PartialEq+Ord{
//where T:numerous+Add+Sub+Ord+Copy+Mul+Div{*/
///converts degrees to radians
///#Examples
///```
///use metropolis::math::*;
///assert_eq!(rad(90.0),PI/2.0);
///```
pub fn rad(x: f32) -> f32 {
    (x / 180.0) * PI
}
///converts radians to degrees
///#Examples
///```
///use metropolis::math::*;
///assert_eq!(deg(PI),180.0);
///```
pub fn deg(x: f32) -> f32 {
    x * 180.0 / PI
}
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
}
///calculates the factorial of a number
///#Examples
///```
///use metropolis::math::*;
///assert_eq!(factorial(6),720);
///```
pub fn factorial(n:u64)->u64{
    real_factorial(n,1)
}
fn real_factorial(n:u64,mut accume:u64)->u64{
    if n==1{
        return accume;
    }
    accume*=n;
    real_factorial(n-1,accume)
}
///converts from one float range to another
///#Examples
///```
///use metropolis::math::*;
///assert_eq!(map(1.0,0.0,2.0,0.0,1.0),0.5);
///```
pub fn map(x:f64,a1:f64,a2:f64,b1:f64,b2:f64)->f64{
    (x-a1)/(a2-a1)*(b2-b1)+b1
}
pub const PI: f32 = 3.14159265359;
pub const TWO_PI: f32 = PI * 2.0;
