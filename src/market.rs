use std::ops::{Add, Div, Mul, Sub};
use super::util::{Date, Range};

pub trait Commodity: std::hash::Hash + Eq + Copy {
    fn into_vec() -> Vec<Self>;
}
#[derive(Debug)]
pub struct Ask<C: Commodity, S: Script> {
    pub quantity: i32,
    pub good: C,
    pub price: S
}
impl<C: Commodity, S: Script> Ask<C, S> {
    pub fn new(good: C, quantity: i32, price: S) -> Self {
        Ask{quantity, good, price}
    }
    pub fn strip(&self)-> (C, i32, S) {
        (self.good.clone(), self.quantity.clone(), self.price.clone())
    }
}
pub struct Bid<C: Commodity, S: Script> {
    pub quantity: i32,
    pub good: C,
    pub price: S
}
impl<C: Commodity, S: Script> Bid<C, S> {
    pub fn new(good: C, quantity: i32, price: S) -> Self {
        Bid{quantity, good, price}
    }
    pub fn strip(&self)-> (C, i32, S) {
        (self.good.clone(), self.quantity.clone(), self.price.clone())
    }
}
pub trait Script: Copy + PartialOrd + Add<Output = Self> + Div<Output = Self> + Mul<Output = Self> + Sub<Output = Self> + Div<f32, Output = Self>{
    const ONE: Self; 
    const ZERO: Self; 
    fn position_in_range(&self, min:&Self, max:&Self) -> f32;
    fn average(&self, other: &Self) -> Self;
}

pub trait Market<C: Commodity, S: Script> {
    fn get_average_historical_price(&self, good: &C, depth: i32) -> Option<S>;
    fn get_average_historical_value(&self, good: &C, depth: i32) -> S;
    fn push_price_history(&mut self, good: C, price: S, date: Date) ;
    fn push_max_unfulfilled_asks_history(&mut self, good: C, price: S, date: Date) ;
    fn push_max_unfulfilled_bids_history(&mut self, good: C, price: S, date: Date) ;
  }

pub trait MarketAgentBasics<C: Commodity, S: Script> {
    fn current_inventory(&self, good: &C) -> i32;
    fn excess_inventory(&self, good: &C) -> i32;
    fn deposit(&mut self, amount: &S) ;
    fn get_lookback(&self) -> i32;
    fn observe_trading_range(&self, good:&C) -> Option<Range<S>>;
    fn max_inventory_capacity(&self, good:&C) -> i32;
    fn receive_good(&mut self, good: &C, quantity: &i32) ;
    fn relinquish_good(&mut self, good: &C, quantity: &i32) -> Result<(), &'static str>;
    fn withdrawl(&mut self, amount: &S) -> Result<(), &'static str>;
}
