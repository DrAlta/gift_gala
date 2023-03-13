//utility functions
pub type Date = i32;

pub struct Range {
    pub min: f32,
    pub max: f32
}
impl Range {
    fn new(min:f32, max:f32) -> Self {
        Range{min, max}
    }
}


pub fn position_in_range(value:f32, min:f32, max:f32) -> f32 {
    (value - min) / (max - min)
}


#[allow(dead_code)]
pub fn position_in_range_clamped(value:f32, min:f32, max:f32) -> f32 {
    let value = (value - min) / (max - min);
    if value < 0_f32 { return 0_f32 };
    if value > 1_f32 { return 1_f32 };
    value
}


pub fn range(selfie:&Vec<f32>)-> Range {
    let mut iter = selfie.into_iter();
    let mut min = iter.next().unwrap_or(&0_f32);
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
