use std::ops::{Add, Div, Mul, Sub};

pub trait Script: std::fmt::Debug + Copy + PartialOrd + Add<Output = Self> + Div<Output = Self> + Mul<Output = Self> + Mul<f32, Output = Self> + Sub<Output = Self> + Div<f32, Output = Self>
//where for<'a> &'a Self: Add<Output = Self> + Div<Output = Self> + Mul<Output = Self> + Sub<Output = Self> 
{
    const ONE: Self; 
    const TWO: Self; 
    const ZERO: Self; 
    fn position_in_range(&self, min:&Self, max:&Self) -> f32;
    fn average(&self, other: &Self) -> Self;
    fn difference(&self, other: &Self) -> Self;
    
}
