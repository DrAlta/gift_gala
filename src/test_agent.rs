use std::ops::Add;
use std::collections::HashMap;

use super::util::{range, Range};
use super::market::{Commodity, Market, MarketAgentBasics, Script};
use super::eerg::EERGAgentBasics;
pub struct TestAgent<C: Commodity, S: Script> {
    //_observedTradingRange is now trading_observations
    inventory: HashMap<C, i32>,
    ideal_inventory: HashMap<C, i32>,
    trading_observations: HashMap<C, Vec<S>>,
    lookback: i32,
    purse: S,
    price_beliefs:HashMap<C, S>,
}
impl<C: Commodity, S: Script> TestAgent<C, S> {
    pub fn def() -> Self {
        TestAgent { inventory: HashMap::new(), ideal_inventory: HashMap::new(), trading_observations: HashMap::new(), lookback: 15, purse: S::ZERO, price_beliefs: HashMap::new() }
    }
    pub fn get_ideal_inventory(&self, good: &C) -> i32 {
        *self.ideal_inventory.get(good).unwrap_or(&0_i32)
    }


    pub fn get_inventory(&self, good: &C) -> i32 {
        *self.inventory.get(good).unwrap_or(&0_i32)
    }
    fn get_believed_price(&self, commodity: &C) -> S {
        *self.price_beliefs.get(commodity). unwrap_or(&S::ZERO)
    }    
}

///EERG functions
impl<C: Commodity, S: Script> TestAgent<C, S> {
    pub fn better_determine_sale_quantity<M: Market<C, S>>(&self, bazaar:&M, commodity_id: &C) -> i32 {
        let mean = bazaar.get_average_historical_value(commodity_id, self.lookback);
        let Some(trading_range) = self.observe_trading_range(commodity_id) else {
            return self.excess_inventory(commodity_id) as i32
        };
        let favorability = mean.position_in_range(&trading_range.min, &trading_range.max);
        //position_in_range: high means price is at a high point
    
        let amount_to_sell = (favorability * self.excess_inventory(commodity_id) as f32) as i32;
        amount_to_sell.max(1)
    }
    //returns amount_to_buy
    pub fn better_determine_purchase_quantity<M: Market<C, S>>(&self, bazaar:&M, commodity:&C) -> i32 {
		let mean = bazaar.get_average_historical_value(commodity,self.lookback);
		let  Some(trading_range) = self.observe_trading_range(commodity) else { return 0 };
		let favorability = mean.position_in_range(&trading_range.min, &trading_range.max);

		//do 1 - favorability to see how close we are to the low 
        let amount_to_buy = ((1_f32 - favorability) * self.max_inventory_capacity(commodity) as f32) as i32;
		amount_to_buy.max(1)
	}
}


impl<C: Commodity, S: Script> MarketAgentBasics<C, S> for TestAgent<C, S> {
    fn deposit(&mut self, amount: &S) {
        self.purse = self.purse + *amount;
    }
    fn withdrawl(&mut self, amount: &S)-> Result<(), &'static str> {
        if !( amount > &self.purse) {
            self.purse = self.purse - *amount;
            Ok(())
        } else {
            Err("insignificant funds")
        }

    }
    fn receive_good(&mut self, good: &C, quantity: &i32) {
        insert_or_add(&mut self.inventory, *good, *quantity);
    }
    fn relinquish_good(&mut self, good: &C, quantity: &i32) -> Result<(), &'static str> {
        insert_or_add(&mut self.inventory, *good, -*quantity);
        Ok(())
    }

    fn excess_inventory(&self, good: &C) -> i32 {
        (self.get_inventory(good) - self.get_ideal_inventory(good)).max(0_i32)
    }
    fn get_lookback(&self) -> i32 {
        self.lookback
    }
    /// bazarrbot returns 0 if he didn't have any
    fn max_inventory_capacity(&self, good:&C) -> i32 {
        (self.get_ideal_inventory(good) - self.get_inventory(good)).max(0_i32)
	}
    fn observe_trading_range(&self, good:&C) -> Option<Range<S>> {
        
        Some(range(self.trading_observations.get(good)?))
    }

}


impl<C: Commodity, S: Script> EERGAgentBasics<C, S> for TestAgent<C, S> {
    fn price_of(&self, commodity: &C) -> S {
        self.get_believed_price(commodity)
    }
}

fn insert_or_add<K: std::hash::Hash + Eq, V: Add<V, Output = V>>(selfie: &mut HashMap<K, V>, key:K, value: V) {
    if let Some(item) = selfie.remove(&key) {
        selfie.insert(key, item + value);
    } else {
        selfie.insert(key, value);
    }
}