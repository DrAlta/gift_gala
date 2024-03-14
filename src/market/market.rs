use crate::{util::Date, Commodity, Script};

pub trait Market<C: Commodity, S: Script>: std::fmt::Debug  {
    fn get_average_historical_price(&self, commodity: &C, depth: i32) -> Option<S>;
    fn get_average_historical_value(&self, commodity: &C, depth: i32) -> S;
    fn push_price_history(&mut self, commodity: C, price: S, date: Date) ;
    fn push_max_unmatched_asks_history(&mut self, commodity: C, price: S, date: Date) ;
    fn push_max_unmatched_bids_history(&mut self, commodity: C, price: S, date: Date) ;
}