use super::util::{Date, Range};
use super::Script;
pub trait Commodity: std::fmt::Debug + std::hash::Hash + Eq + Copy {
    fn into_vec() -> Vec<Self>;
}
#[derive(Debug)]
pub struct Ask<C: Commodity, S: Script> {
    pub quantity: i32,
    pub commodity: C,
    pub price: S
}
impl<C: Commodity, S: Script> Ask<C, S> {
    pub fn new(commodity: C, quantity: i32, price: S) -> Self {
        Ask{quantity, commodity, price}
    }
    pub fn strip(&self)-> (C, i32, S) {
        (self.commodity.clone(), self.quantity.clone(), self.price.clone())
    }
}
pub struct Bid<C: Commodity, S: Script> {
    pub quantity: i32,
    pub commodity: C,
    pub price: S
}
impl<C: Commodity, S: Script> Bid<C, S> {
    pub fn new(commodity: C, quantity: i32, price: S) -> Self {
        Bid{quantity, commodity, price}
    }
    pub fn strip(&self)-> (C, i32, S) {
        (self.commodity.clone(), self.quantity.clone(), self.price.clone())
    }
}


pub trait Market<C: Commodity, S: Script> {
    fn get_average_historical_price(&self, commodity: &C, depth: i32) -> Option<S>;
    fn get_average_historical_value(&self, commodity: &C, depth: i32) -> S;
    fn push_price_history(&mut self, commodity: C, price: S, date: Date) ;
    fn push_max_unmatched_asks_history(&mut self, commodity: C, price: S, date: Date) ;
    fn push_max_unmatched_bids_history(&mut self, commodity: C, price: S, date: Date) ;
  }

pub trait MarketAgentBasics<C: Commodity, S: Script> {
    fn current_inventory(&self, commodity: &C) -> i32;
    fn excess_inventory(&self, commodity: &C) -> i32;
    fn deposit(&mut self, amount: &S) ;
    fn get_lookback(&self) -> i32;
    fn observe_trading_range(&self, commodity:&C) -> Option<Range<S>>;
    fn max_inventory_capacity(&self, commodity:&C) -> i32;
    fn receive_good(&mut self, commodity: &C, quantity: &i32) ;
    fn relinquish_good(&mut self, commodity: &C, quantity: &i32) -> Result<(), &'static str>;
    fn withdrawl(&mut self, amount: &S) -> Result<(), &'static str>;
}
