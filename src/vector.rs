use std::ops::Mul;
use std::ops::Div;
use std::ops::Add;
use std::ops::Sub;
use std::ops::AddAssign;
use std::ops::MulAssign;
use std::ops::SubAssign;
use std::ops::DivAssign;
use std::fmt;
use std::fmt::Display;
#[derive(Clone,Debug)]
pub struct Vector<T:Clone+std::marker::Copy+Display>{
    pub vec:Vec<T>,
}
impl<T:Clone+Display+Copy+Eq+Ord+Mul+AddAssign+Mul<T, Output = f32>> Vector<T>{
    pub fn dot(self,vec1:Vector<T>)->Option<f32>{
        let mut x=0f32;
        if self.vec.len()!=vec1.vec.len(){
            return None;
        }
        for v in 0..self.vec.len(){
            x+=self.vec[v]*vec1.vec[v];
        }
        Some(x)
    }
}
impl<T:Clone+Display+std::marker::Copy+Mul+Mul<T, Output = T>> Mul<T> for Vector<T>{
    type Output = Self;
    fn mul(self,mul1:T)->Self{
        let mut vec1 = Vector{vec:vec![]};
        for t in self.vec{
            vec1.vec.push(t *mul1);
        }   
        vec1
    }
}
impl<T:Clone+Display+std::marker::Copy+Add+Add<T, Output = T>> Add<Vector<T>> for Vector<T>{
    type Output = Self;
    fn add(self,add1:Vector<T>)->Self{
        let mut vec1 = Vector{vec:vec![]};
        let mut c = 0;
        for t in self.vec{
            vec1.vec.push(t +add1.vec[c]);
            c+=1;
        }   
        vec1
    }
}
impl<T:Clone+Display+std::marker::Copy+Add+Add<T, Output = T>> Add<T> for Vector<T>{
    type Output = Self;
    fn add(self,add1:T)->Self{
        let mut vec1 = Vector{vec:vec![]};
        for t in self.vec{
            vec1.vec.push(t +add1);
        }   
        vec1
    }
}
impl<T:Clone+Display+std::marker::Copy+Sub+Sub<T, Output = T>> Sub<T> for Vector<T>{
    type Output = Self;
    fn sub(self,sub1:T)->Self{
        let mut vec1 = Vector{vec:vec![]};
        for t in self.vec{
            vec1.vec.push(t -sub1);
        }   
        vec1
    }
}
impl<T:Clone+Display+std::marker::Copy+Div+Div<T, Output = T>> Div<T> for Vector<T>{
    type Output = Self;
    fn div(self,div1:T)->Self{
        let mut vec1 = Vector{vec:vec![]};
        for t in self.vec{
            vec1.vec.push(t /div1);
        }   
        vec1
    }
}
impl<T:Clone+Display+std::marker::Copy+Add+Add<T, Output = T>+AddAssign> AddAssign<T> for Vector<T>{
    fn add_assign(&mut self,add1:T){
        let mut changer = self.clone();
        for t in &mut changer.vec{
            *t+=add1;
        }
        *self = changer;
    }
}
impl<T:Clone+Display+std::marker::Copy+Mul+Mul<T, Output = T>+MulAssign> MulAssign<T> for Vector<T>{
    fn mul_assign(&mut self,mul1:T){
        let mut changer = self.clone();
        for t in &mut changer.vec{
            *t*=mul1;
        }
        *self = changer;
    }
}
impl<T:Clone+Display+std::marker::Copy+Sub+Sub<T, Output = T>+SubAssign> SubAssign<T> for Vector<T>{
    fn sub_assign(&mut self,sub1:T){
        let mut changer = self.clone();
        for t in &mut changer.vec{
            *t-=sub1;
        }
        *self = changer;
    }
}
impl<T:Clone+Display+std::marker::Copy+Div+Div<T, Output = T>+DivAssign> DivAssign<T> for Vector<T>{
    fn div_assign(&mut self,div1:T){
        let mut changer = self.clone();
        for t in &mut changer.vec{
            *t/=div1;
        }
        *self = changer;
    }
}
impl<T:Clone+std::marker::Copy+Display> fmt::Display for Vector<T>{
    fn fmt(&self,f: &mut fmt::Formatter<'_>)->fmt::Result{
        for i in self.vec.clone(){
            write!(f, "{}", i)?;
            }
        Ok(())
    }
}
