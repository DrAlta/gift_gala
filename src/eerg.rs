use super::market::{Commodity, Market, MarketAgentBasics, Script};

pub trait EERGAgentBasics<C: Commodity, S: Script> {
    fn price_of(&self, commodity: &C) -> S ;
}
pub trait EERGAgent<C: Commodity, S: Script> {
    fn determine_purchase_quantity<M: Market<C, S>>(&self, bazaar:&M, commodity: &C) -> i32;
    fn determine_sale_quantity<M: Market<C, S>>(&self, bazaar:&M, commodity_id: &C) -> i32;
}
impl<T: MarketAgentBasics<C, S>+ EERGAgentBasics<C, S>, C: Commodity, S: Script> EERGAgent<C, S> for T {
    fn determine_sale_quantity<M: Market<C, S>>(&self, bazaar:&M, commodity_id: &C) -> i32 {
        let Some(mean) = bazaar.get_average_historical_price(commodity_id, self.get_lookback()) else {
            return 0
        };
        let Some(trading_range) = self.observe_trading_range(commodity_id) else {
            return 0
        };
        let favorability = mean.position_in_range(&trading_range.min, &trading_range.max);
        //position_in_range: high means price is at a high point
    
        let amount_to_sell = (favorability * self.excess_inventory(commodity_id)) as i32;
        amount_to_sell.max(1)
    }
    //returns amount_to_buy
    fn determine_purchase_quantity<M: Market<C, S>>(&self, bazaar:&M, commodity:&C) -> i32 {
		let Some(mean) = bazaar.get_average_historical_price(commodity,self.get_lookback()) else {
            return 0
        };
		let  Some(trading_range) = self.observe_trading_range(commodity) else {
            return 0
        };
		let favorability = mean.position_in_range(&trading_range.min, &trading_range.max);

		//do 1 - favorability to see how close we are to the low 
        let amount_to_buy = ((1_f32 - favorability) * self.max_inventory_capacity(commodity)) as i32;
		amount_to_buy.max(1)
	}

}
