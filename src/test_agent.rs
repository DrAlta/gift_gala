use std::collections::HashMap;
use super::util::{position_in_range, range, Range};
use super::market::{Commodity, Market, MarketAgentBasics};
pub struct TestAgent<C: Commodity> {
    //_observedTradingRange is now trading_observations
    inventory: HashMap<C, f32>,
    ideal_inventory: HashMap<C, f32>,
    trading_observations: HashMap<C, Vec<f32>>,
    lookback: i32,
}
impl<C: Commodity> TestAgent<C> {
    pub fn def() -> Self {
        TestAgent { inventory: HashMap::new(), ideal_inventory: HashMap::new(), trading_observations: HashMap::new(), lookback: 15 }
    }
    pub fn get_ideal_inventory(&self, good: &C) -> f32 {
        *self.ideal_inventory.get(good).unwrap_or(&0_f32)
    }


    pub fn get_inventory(&self, good: &C) -> f32 {
        *self.inventory.get(good).unwrap_or(&0_f32)
    }    
}

///EERG functions
impl<C: Commodity> TestAgent<C> {
    pub fn better_determine_sale_quantity<M: Market<C>>(&self, bazaar:&M, commodity_id: &C) -> i32 {
        let mean = bazaar.get_average_historical_value(commodity_id, self.lookback);
        let Some(trading_range) = self.observe_trading_range(commodity_id) else {
            return self.excess_inventory(commodity_id) as i32
        };
        let favorability = position_in_range(mean, trading_range.min, trading_range.max);
        //position_in_range: high means price is at a high point
    
        let amount_to_sell = (favorability * self.excess_inventory(commodity_id)) as i32;
        amount_to_sell.max(1)
    }
    //returns amount_to_buy
    pub fn better_determine_purchase_quantity<M: Market<C>>(&self, bazaar:&M, commodity:&C) -> i32 {
		let mean = bazaar.get_average_historical_value(commodity,self.lookback);
		let  Some(trading_range) = self.observe_trading_range(commodity) else { return 0 };
		let favorability = position_in_range(mean, trading_range.min, trading_range.max);

		//do 1 - favorability to see how close we are to the low 
        let amount_to_buy = ((1_f32 - favorability) * self.max_inventory_capacity(commodity)) as i32;
		amount_to_buy.max(1)
	}
}


impl<C: Commodity> MarketAgentBasics<C> for TestAgent<C> {
    fn excess_inventory(&self, good: &C) -> f32 {
        (self.get_inventory(good) - self.get_ideal_inventory(good)).max(0_f32)
    }
    fn get_lookback(&self) -> i32 {
        self.lookback
    }
    /// bazarrbot returns 0 if he didn't have any
    fn max_inventory_capacity(&self, good:&C) -> f32 {
        (self.get_ideal_inventory(good) - self.get_inventory(good)).max(0_f32)
	}
    fn observe_trading_range(&self, good:&C) -> Option<Range> {
        
        Some(range(self.trading_observations.get(good)?))
    }

}