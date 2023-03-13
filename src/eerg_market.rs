//use std::collections::HashMap;
use super::history_log::HistoryLog;
use super::market::{CommodityID, Market};
use super::util::Date;
pub struct EERGMarket {
    price_history: HistoryLog<CommodityID, f32>,
    /// log of the max bid for days what there are no asks but there where bids 
    max_unfulfilled_bids_history: HistoryLog<CommodityID, f32>,
}
impl EERGMarket {
   pub fn def()-> Self {
        EERGMarket { price_history: HistoryLog::def(), max_unfulfilled_bids_history: HistoryLog::def() }
    }
    pub fn push_price_history(&mut self, good: CommodityID, price: f32, date: Date) {
        self.price_history.push(good, price, date)
    }
    pub fn push_max_unfulfilled_bids_history(&mut self, good: CommodityID, price: f32, date: Date) {
        self.max_unfulfilled_bids_history.push(good, price, date)
    }
}
impl Market for EERGMarket {


    fn get_average_historical_price(&self, good: &CommodityID, depth: i32) -> Option<f32> {
        let prices = self.price_history.get_values(good)?;
        let size = prices.len();
        let range = size.min(depth as usize);
        let mut total = 0_f32;
        for idx in size - range .. size {
            total += prices[idx];
        }
        Some(total / (range as f32))
    }
    fn get_average_historical_value(&self, good: &CommodityID, depth: i32) -> f32 {
        if let Some(price) = self.get_average_historical_price(good, depth) {
            return price
        };

        let Some(bids) = self.max_unfulfilled_bids_history.get_values(good) else {
            return 0_f32
        };
        let size = bids.len();
        let range = size.min(depth as usize);
        let mut total = 0_f32;
        for idx in size - range .. size {
            total += bids[idx];
        }
        total / (range as f32)
    }
}
