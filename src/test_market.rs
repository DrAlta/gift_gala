//use std::collections::HashMap;
use super::history_log::HistoryLog;
use super::Script;
use super::market::{Commodity, Market};
use super::util::Date;

#[derive(Debug)]
pub struct TestMarket<C: Commodity, S: Script> {
    price_history: HistoryLog<C, S>,
    /// log of the max bid for days what there are no asks but there where bids 
    max_unmatched_bids_history: HistoryLog<C, S>,
}
impl<C: Commodity, S: Script> TestMarket<C, S> {
   pub fn def()-> Self {
        TestMarket { price_history: HistoryLog::def(), max_unmatched_bids_history: HistoryLog::def() }
    }
}
impl<C: Commodity, S: Script> Market<C, S> for TestMarket<C, S> {
    fn push_price_history(&mut self, commodity: C, price: S, date: Date) {
        self.price_history.push(commodity, price, date)
    }
    fn push_max_unmatched_asks_history(&mut self, commodity: C, price: S, date: Date) {
        self.max_unmatched_bids_history.push(commodity, price, date)
    }
    fn push_max_unmatched_bids_history(&mut self, commodity: C, price: S, date: Date) {
        self.max_unmatched_bids_history.push(commodity, price, date)
    }


    fn get_average_historical_price(&self, commodity: &C, depth: i32) -> Option<S> {
        let prices = self.price_history.get_values(commodity)?;
        let size = prices.len();
        let range = size.min(depth as usize);
        let mut total = S::ZERO;
        for idx in size - range .. size {
            total = total + prices[idx];
        }
        Some(total / (range as f32))
    }
    fn get_average_historical_value(&self, commodity: &C, depth: i32) -> S {
        if let Some(price) = self.get_average_historical_price(commodity, depth) {
            return price
        };

        let Some(bids) = self.max_unmatched_bids_history.get_values(commodity) else {
            return S::ZERO
        };
        let size = bids.len();
        let range = size.min(depth as usize);
        let mut total = S::ZERO;
        for idx in size - range .. size {
            total = total + bids[idx];
        }
        total / (range as f32)
    }
}

