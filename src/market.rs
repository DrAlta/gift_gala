use std::collections::HashMap;
use super::util::{position_in_range, range, Range};
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[allow(dead_code)]
pub enum CommodityID {
    Stuff,
    Things,
}

pub struct EERGAgent {
    //_observedTradingRange is now trading_observations
    inventory: HashMap<CommodityID, f32>,
    ideal_inventory: HashMap<CommodityID, f32>,
    trading_observations: HashMap<CommodityID, Vec<f32>>,
    lookback: i32,
}
impl EERGAgent {
    pub fn def() -> Self {
        EERGAgent { inventory: HashMap::new(), ideal_inventory: HashMap::new(), trading_observations: HashMap::new(), lookback: 15 }
    }
    pub fn get_ideal_inventory(&self, good: &CommodityID) -> f32 {
        *self.ideal_inventory.get(good).unwrap_or(&0_f32)
    }


    pub fn get_inventory(&self, good: &CommodityID) -> f32 {
        *self.inventory.get(good).unwrap_or(&0_f32)
    }



    
    
}


pub trait Market {
    fn get_average_historical_price(&self, good: &CommodityID, depth: i32) -> Option<f32>;
    fn get_average_historical_value(&self, good: &CommodityID, depth: i32) -> f32;
}

pub trait MarketAgentBasics {
    fn get_lookback(&self) -> i32;
    fn observe_trading_range(&self, good:&CommodityID) -> Option<Range>;
    fn excess_inventory(&self, good: &CommodityID) -> f32;
    fn max_inventory_capacity(&self, good:&CommodityID) -> f32;
    
}

///
impl MarketAgentBasics for EERGAgent {
    fn excess_inventory(&self, good: &CommodityID) -> f32 {
        (self.get_inventory(good) - self.get_ideal_inventory(good)).max(0_f32)
    }
    fn get_lookback(&self) -> i32 {
        self.lookback
    }
    /// bazarrbot returns 0 if he didn't have any
    fn max_inventory_capacity(&self, good:&CommodityID) -> f32 {
        (self.get_ideal_inventory(good) - self.get_inventory(good)).max(0_f32)
	}
    fn observe_trading_range(&self, good:&CommodityID) -> Option<Range> {
        
        Some(range(self.trading_observations.get(good)?))
    }

}
///EERG

//new stuff
impl EERGAgent {
    pub fn better_determine_sale_quantity<M: Market>(&self, bazaar:&M, commodity_id: &CommodityID) -> i32 {
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
    pub fn better_determine_purchase_quantity<M: Market>(&self, bazaar:&M, commodity:&CommodityID) -> i32 {
		let mean = bazaar.get_average_historical_value(commodity,self.lookback);
		let  Some(trading_range) = self.observe_trading_range(commodity) else { return 0 };
		let favorability = position_in_range(mean, trading_range.min, trading_range.max);

		//do 1 - favorability to see how close we are to the low 
        let amount_to_buy = ((1_f32 - favorability) * self.max_inventory_capacity(commodity)) as i32;
		amount_to_buy.max(1)
	}
}
