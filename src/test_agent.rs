use std::ops::{Add, Div, Mul, Sub};
use std::collections::HashMap;

use crate::{EERGAgentBasics, Script};

use crate::util::{range, Range};
use crate::market::{BaseValues, Commodity, Market, MarketAgentBasics};


pub struct TestAgent<C: Commodity, S: Script> {
    //_observedTradingRange is now trading_observations
    inventory: HashMap<C, i32>,
    ideal_inventory: HashMap<C, i32>,
    trading_observations: HashMap<C, Vec<S>>,
    lookback: i32,
    purse: S,
    price_beliefs:HashMap<C, Range<S>>,
}


impl<C: Commodity, S: Script> BaseValues<C, S> for TestAgent<C, S>
where for<'a> &'a S: Add<Output = S> + Div<Output = S> + Mul<Output = S> + Sub<Output = S>
{
    fn get_base_value(&self, _commodity: &C) -> S {
        S::ONE
    }
    
    
    fn get_base_value_range(&self, _commodity: &C) -> Range<S> {
        Range::new(S::ONE, S::TWO)
    }
}

impl<C: Commodity, S: Script> TestAgent<C, S> {
    pub fn def() -> Self {
        TestAgent { inventory: HashMap::new(), ideal_inventory: HashMap::new(), trading_observations: HashMap::new(), lookback: 15, purse: S::ZERO, price_beliefs: HashMap::new() }
    }
    pub fn get_ideal_inventory(&self, commodity: &C) -> i32 {
        *self.ideal_inventory.get(commodity).unwrap_or(&0_i32)
    }


    pub fn get_inventory(&self, commodity: &C) -> i32 {
        *self.inventory.get(commodity).unwrap_or(&0_i32)
    }
    fn get_believed_price(&self, commodity: &C) -> S {
        if let Some(range) = self.price_beliefs.get(commodity) {
            range.max.average(&range.min)
        } else {
            S::ONE
        }
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
    fn receive_good(&mut self, commodity: &C, quantity: &i32) {
        insert_or_add(&mut self.inventory, *commodity, *quantity);
    }
    fn relinquish_good(&mut self, commodity: &C, quantity: &i32) -> Result<(), &'static str> {
        insert_or_add(&mut self.inventory, *commodity, -*quantity);
        Ok(())
    }

    fn excess_inventory(&self, commodity: &C) -> i32 {
        (self.get_inventory(commodity) - self.get_ideal_inventory(commodity)).max(0_i32)
    }
    fn current_inventory(&self, commodity: &C) -> i32 {
        self.get_inventory(commodity)
    }
    fn get_lookback(&self) -> i32 {
        self.lookback
    }
    /// bazarrbot returns 0 if he didn't have any
    fn max_inventory_capacity(&self, commodity:&C) -> i32 {
        (self.get_ideal_inventory(commodity) - self.get_inventory(commodity)).max(0_i32)
	}
    fn observe_trading_range(&self, commodity:&C) -> Option<Range<S>> {
        
        Some(range(self.trading_observations.get(commodity)?))
    }

}


impl<C: Commodity, S: Script> EERGAgentBasics<C, S> for TestAgent<C, S> {
    fn get_price_beliefs(&self, commodity:&C) -> Option<Range<S>> {
        if let Some(thing) = self.price_beliefs.get(commodity) {
            Some(*thing)
        } else {
            None
        }
    }
    fn price_of(&self, commodity: &C) -> S {
        self.get_believed_price(commodity)
    }
    fn set_price_beliefs(&mut self, commodity:&C, belief: Range<S>){
        self.price_beliefs.insert(*commodity, belief);
    }
}

fn insert_or_add<K: std::hash::Hash + Eq, V: Add<V, Output = V>>(selfie: &mut HashMap<K, V>, key:K, value: V) {
    if let Some(item) = selfie.remove(&key) {
        selfie.insert(key, item + value);
    } else {
        selfie.insert(key, value);
    }
}