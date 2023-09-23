use std::cmp::Ordering::{Greater, Less};
use super::market::{Ask, Bid, Commodity, Market, MarketAgentBasics};

use super::util::Range;

use super::Script;

pub trait EERGAgentBasics<C: Commodity, S: Script> {
    fn get_price_beliefs(&self, commodity:&C) -> Option<Range<S>>;
    fn price_of(&self, commodity: &C) -> S ;
    fn set_price_beliefs(&mut self, commodity:&C, belief: Range<S>);
    }
pub trait EERGAgent<C: Commodity, S: Script> {
    fn get_average_historical_price<M: Market<C, S>>(&self, bazaar:&M, commodity_id: &C) -> Option<S>;

    fn determine_purchase_quantity<M: Market<C, S>>(&self, bazaar:&M, commodity: &C) -> i32;
    fn determine_sale_quantity<M: Market<C, S>>(&self, bazaar:&M, commodity_id: &C) -> i32;
    fn create_ask<M: Market<C, S>>(&self, bazaar:&M, commodity:&C, limit: i32) -> Option<Ask<C, S>>;
    fn create_bid<M: Market<C, S>>(&self, bazaar:&M, commodity: &C, limit:i32) -> Option<Bid<C, S>>;
    fn price_update_from_ask<M: Market<C, S>>(
        &mut self, 
        amount_offered: i32, 
        commodity: &C, 
        offered_price: S, 
        sold_for: Vec<S>, 
        quantity_sold: i32, 
        market: &M,
        supply_cmp_demand: &std::cmp::Ordering,
        total_ammount_sold_this_round: i32
    );
    fn price_update_from_bid<M: Market<C, S>>(
        &mut self, 
        amount_offered: i32, 
        commodity: &C, 
        offered_price: S, 
        sold_for: Vec<S>, 
        quantity_sold: i32, 
        market: &M,
        supply_cmp_demand: &std::cmp::Ordering,
        total_ammount_sold_this_round: i32
    );
}
impl<T: MarketAgentBasics<C, S>+ EERGAgentBasics<C, S>, C: Commodity, S: Script> EERGAgent<C, S> for T {
    #[allow(dead_code)]
    fn get_average_historical_price<M: Market<C, S>>(&self, bazaar:&M, commodity_id: &C) -> Option<S>{
        bazaar.get_average_historical_price(commodity_id, self.get_lookback())
    }

    fn create_ask<M: Market<C, S>>(&self, bazaar:&M, commodity:&C, limit: i32) -> Option<Ask<C, S>> {
		let ask_price = self.price_of(commodity);
		let ideal = self.determine_sale_quantity(bazaar, commodity);

		//can't sell less than limit
		let quantity_to_sell = ideal.min(limit);
		if quantity_to_sell > 0 {
			return Some(Ask::new(*commodity, quantity_to_sell, ask_price))
		};
		return None;
	}
    fn create_bid<M: Market<C, S>>(&self, bazaar:&M, commodity: &C, limit:i32) -> Option<Bid<C, S>> {
        let bid_price = self.price_of(commodity);
        let ideal = self.determine_purchase_quantity(bazaar, commodity);

        //can't buy more than limit
        let quantity_to_buy = ideal.min(limit);
        if quantity_to_buy > 0 {
            return Some(Bid::new(*commodity, quantity_to_buy, bid_price));
        }
        None
    }
    fn determine_sale_quantity<M: Market<C, S>>(&self, bazaar:&M, commodity_id: &C) -> i32 {
        let Some(mean) = bazaar.get_average_historical_price(commodity_id, self.get_lookback()) else {
            logy("No historical avg price");
            return 0
        };
        let Some(trading_range) = self.observe_trading_range(commodity_id) else {
            logy("no trading fange");
            return 0
        };
        let favorability = mean.position_in_range(&trading_range.min, &trading_range.max);
        //position_in_range: high means price is at a high point
    
        let amount_to_sell = (favorability * self.excess_inventory(commodity_id) as f32) as i32;
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
        let amount_to_buy = ((1_f32 - favorability) * self.max_inventory_capacity(commodity) as f32) as i32;
		amount_to_buy.max(1)
	}
    fn price_update_from_ask<M: Market<C, S>>(
        &mut self, 
        amount_offered: i32, 
        commodity: &C, 
        offered_price: S, 
        sold_for: Vec<S>, 
        quantity_sold: i32, 
        market: &M,
        supply_cmp_demand: &std::cmp::Ordering,
        total_ammount_of_commodit_sold_this_round: i32
    ) {
        let market_share = (quantity_sold as f32) / (total_ammount_of_commodit_sold_this_round as f32);
        let weight = (quantity_sold as f32) / (amount_offered as f32);
        let historic_mean_price = market
            .get_average_historical_price(commodity, self.get_lookback())
            .unwrap_or(S::ONE)
        ;
        let displacement = historic_mean_price * weight;
        let mut belief = self.get_price_beliefs(commodity).unwrap_or(Range::new(S::ONE, S::ONE));
        if quantity_sold == 0 {
            belief.shift(displacement * (-1.0 / 6.0));
        } else {
            for traded_price in sold_for{
                if market_share < 0.75 {
                    belief.shift(displacement * (-1.0 / 7.0));
                } else if offered_price < traded_price {
                    belief.shift((traded_price - offered_price) * (weight * 1.2));      
                } else if supply_cmp_demand == &Less {
                    belief.shift( historic_mean_price * 0.2);
                } else {
                    belief.shift( historic_mean_price * -0.2);
                }
            }// end for traded_price in sold_for
        }
    }
    fn price_update_from_bid<M: Market<C, S>>(
        &mut self, 
        amount_offered: i32, 
        commodity: &C, 
        offered_price: S, 
        prices_sold_for: Vec<S>, 
        quantity_sold: i32, 
        market: &M,
        supply_cmp_demand: &std::cmp::Ordering,
        other_agent_sold_this_commodity: &bool
    ) {
  
        let mean = market
            .get_average_historical_price(commodity, self.get_lookback())
            .unwrap_or(S::ONE)
        ;

        let mut belief = self.get_price_beliefs(commodity).unwrap_or(Range::new(S::ONE, S::ONE));
        if quantity_sold == 0 {
            belief.max = belief.max * 1.1;
            if self.current_inventory(commodity) < (self.max_inventory_capacity(commodity) / 4 ) 
            {
                let displacement = offered_price.difference(&mean) / mean;
                belief.shift(displacement);
            } else if supply_cmp_demand == &Greater && offered_price > mean {
                belief.shift((offered_price - mean) * -1.1);
            } else if supply_cmp_demand == &Greater {
                belief.shift(mean * 0.2);
            } else {
                belief.shift(mean * -0.2);
            } 
        } else {
            for traded_price in prices_sold_for {
                let percent_of_order_sold = (quantity_sold as f32) / (amount_offered as f32);
                if percent_of_order_sold >= 0.5 {
                    belief.max = belief.max * 0.9;
                } else {
                    belief.max = belief.max * 1.1;
                }

                if (!other_agent_sold_this_commodity) && (self.current_inventory(commodity) < (self.max_inventory_capacity(commodity) / 4 )) {
                    let displacement = traded_price.difference(&mean) / mean;
                    belief.shift(displacement);
                } else if offered_price > traded_price {
                   belief.shift((offered_price - traded_price) * -1.1);
                } else if supply_cmp_demand == &Greater && offered_price > mean {
                    belief.shift((traded_price - mean) * -1.1);
                } else if supply_cmp_demand == &Greater {
                    belief.shift(mean * 0.2);
                } else {
                    belief.shift(mean * -0.2);
                }
            }// end for traded_price in prices_sold_for
        }
        self.set_price_beliefs(commodity, belief);
    }

}

fn logy(msg:&str) {
    println!("{}", msg)
}