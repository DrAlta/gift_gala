//utility functions
pub type Date = i32;

pub struct Range<T: PartialOrd> {
    pub min: T,
    pub max: T
}
impl<T: PartialOrd> Range<T> {
    fn new(min:T, max:T) -> Self {
        Range{min, max}
    }
}



#[allow(dead_code)]
pub fn position_in_range_clamped(value:f32, min:f32, max:f32) -> f32 {
    let value = (value - min) / (max - min);
    if value < 0_f32 { return 0_f32 };
    if value > 1_f32 { return 1_f32 };
    value
}

use super::market::Script;
pub fn range<T: Script>(selfie:&Vec<T>)-> Range<T> {
    let zero = &T::ZERO;
    let mut iter = selfie.into_iter();
    let mut min = iter.next().unwrap_or(zero);
    let mut max = min;
    for x in iter {
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
    }
    Range::new(*min, *max)
}
