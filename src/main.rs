use vulkanoing::*;
//use vulkanoing::Canvas;
fn main() {
    size(800,600);
    show();
}
/*
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
fn map<T>(point:[T;2],scale:[T;2])->[f32;2]
    where T:Add+Sub+Div+Mul+Sub<T, Output =f32>+Add<T, Output =f32>+Div<T, Output = f32>+Mul<T, Output = f32>+Copy+Clone+PartialEq{
        let new_point:[f32;2] = [(point[0]/scale[0]) as f32,(point[1]/scale[1]) as f32];
        [(new_point[0]/2.0)-1.0,(new_point[1]/2.0)-1.0]
}
fn map_colors<T>(point:[T;2],scale:[T;2])->[f32;2]
    where T:Add+Sub+Div+Mul+Sub<T, Output =f32>+Add<T, Output =f32>+Div<T, Output = f32>+Mul<T, Output = f32>+Copy+Clone+PartialEq{
        [(point[0]/scale[0]) as f32,(point[1]/scale[1]) as f32]
}*/
