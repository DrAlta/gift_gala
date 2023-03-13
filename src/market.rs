use super::util::Range;

pub trait Commodity: std::hash::Hash + Eq + Clone {

}

pub trait Market<C: Commodity> {
    fn get_average_historical_price(&self, good: &C, depth: i32) -> Option<f32>;
    fn get_average_historical_value(&self, good: &C, depth: i32) -> f32;
}

pub trait MarketAgentBasics<C: Commodity> {
    fn get_lookback(&self) -> i32;
    fn observe_trading_range(&self, good:&C) -> Option<Range>;
    fn excess_inventory(&self, good: &C) -> f32;
    fn max_inventory_capacity(&self, good:&C) -> f32;
    
}
