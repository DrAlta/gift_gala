use super::util::{Date, Range};
use super::Script;
pub trait Commodity: std::fmt::Debug + std::hash::Hash + Eq + Copy {
    fn into_vec() -> Vec<Self>;
}
#[derive(Debug)]
pub struct Ask<C: Commodity, S: Script> {
    pub amount_put_up: i32,
    pub commodity: C,
    pub trade_price: S
}
impl<C: Commodity, S: Script> Ask<C, S> {
    pub fn new(commodity: C, quantity: i32, price: S) -> Self {
        Ask{amount_put_up: quantity, commodity, trade_price: price}
    }
    pub fn strip(&self)-> (C, i32, S) {
        (self.commodity.clone(), self.amount_put_up.clone(), self.trade_price.clone())
    }
}
pub struct Bid<C: Commodity, S: Script> {
    pub amount_wanted: i32,
    pub commodity: C,
    pub value_of_acquiring: S
}
impl<C: Commodity, S: Script> Bid<C, S> {
    pub fn new(commodity: C, quantity: i32, price: S) -> Self {
        Bid{amount_wanted: quantity, commodity, value_of_acquiring: price}
    }
    pub fn strip(&self)-> (C, i32, S) {
        (self.commodity.clone(), self.amount_wanted.clone(), self.value_of_acquiring.clone())
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
