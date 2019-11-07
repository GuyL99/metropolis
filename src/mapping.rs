/*use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;*/
pub fn map(point:[u16;2],scale:[u16;2])->[f32;2]{
    //where T:Add+Sub+Div+Mul+Sub<T, Output =T>+Add<T, Output =T>+Div<T, Output = T>+Mul<T, Output = T>+Copy+Clone+PartialEq{
        let new_point:[f32;2] = [(point[0] as f32/scale[0] as f32),(point[1] as f32/scale[1] as f32) ];
        [(new_point[0]*2.0)-1.0,(new_point[1]*2.0)-1.0]
}
pub fn map_colors(color:[u8;4])->[f32;4]{
    //where T:Add+Sub+Div+Mul+Sub<T, Output =f32>+Add<T, Output =f32>+Div<T, Output = f32>+Mul<T, Output = f32>+Copy+Clone+PartialEq{
        [color[0] as f32/255.0,color[1] as f32/255.0,color[2] as f32/255.0,color[3]as f32/255.0]
}
