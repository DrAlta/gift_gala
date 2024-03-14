use std::ops::Add;
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Range<T: PartialOrd> {
    pub min: T,
    pub max: T
}
impl<T: PartialOrd> Range<T> {
    pub fn new(min:T, max:T) -> Self {
        Range{min, max}
    }
    #[allow(dead_code)]
    pub fn set_max(&mut self, new_max: T) {
        self.max = new_max;
    }
    #[allow(dead_code)]
    pub fn set_min(&mut self, new_min: T) {
        self.min = new_min;
    }
}
impl<T: PartialOrd + Clone> Range<T> {

    #[allow(dead_code)]
    fn get_max(&mut self) -> T {
        self.max.clone()
    }
    #[allow(dead_code)]
    fn get_min(&mut self) -> T {
        self.min.clone()
    }
}


impl<T: PartialOrd + Add<T, Output = T> + Copy> Range<T> {
    #[allow(dead_code)]
    pub fn shift(&mut self, amount: T) {
        self.max = self.max + amount;
        self.min = self.min + amount;
    }
}




use crate::Script;
impl<S:Script> Range<S>{
    pub fn contract(&mut self, amount: S) {
        let half_of_amount = amount / S::TWO;
        let new_max = self.max - half_of_amount;
        let new_min = self.min + half_of_amount;
        if new_max < new_min {
            let avg = S::average(&new_max, &new_min);
            self.max = avg.clone();
            self.min = avg;
        } else {
            self.max = new_max;
            self.min = new_min;
        }
    }
    pub fn mean(&self) -> S {
        S::average(&self.max, &self.min)
    }
    #[allow(dead_code)]
    pub fn shift_towards (&mut self, target: S, amount: S) {
        let towards_target = target - self.max.average(&self.min);
        if towards_target > S::ZERO {
            if towards_target < amount {
                self.shift(towards_target);
            } else {
                self.shift(amount)
            }
        } else {
            if S::ZERO - towards_target < amount {
                self.shift(towards_target);
            } else {
                self.shift(S::ZERO - amount)
            }
        }
    }

}

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
#[allow(dead_code)]
pub fn position_in_range_clamped(value:f32, min:f32, max:f32) -> f32 {
    let value = (value - min) / (max - min);
    if value < 0_f32 { return 0_f32 };
    if value > 1_f32 { return 1_f32 };
    value
}
