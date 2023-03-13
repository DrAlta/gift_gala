use std::ops::{Add, Div, Mul, Sub};
use super::util::Range;

pub trait Commodity: std::hash::Hash + Eq + Clone {

}

pub trait Script: Copy + PartialOrd + Add<Output = Self> + Div<Output = Self> + Mul<Output = Self> + Sub<Output = Self> + Div<f32, Output = Self>{
    const ZERO: Self; 
    fn position_in_range(&self, min:&Self, max:&Self) -> f32;
}

pub trait Market<C: Commodity, S: Script> {
    fn get_average_historical_price(&self, good: &C, depth: i32) -> Option<S>;
    fn get_average_historical_value(&self, good: &C, depth: i32) -> S;
}

pub trait MarketAgentBasics<C: Commodity, S: Script> {
    fn get_lookback(&self) -> i32;
    fn observe_trading_range(&self, good:&C) -> Option<Range<S>>;
    fn excess_inventory(&self, good: &C) -> f32;
    fn max_inventory_capacity(&self, good:&C) -> f32;
    
}
