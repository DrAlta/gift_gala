use crate::{util::Range, Script};

use super::Commodity;

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
