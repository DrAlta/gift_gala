use std::{cmp::Ordering, ops::{Add, Div, Mul, Sub}};

use qol::logy;

use crate::{market::{Ask, Bid, Commodity, Market, MarketAgentBasics}, util::Range, EERGAgentBasics, Script};

#[derive(Debug)]
pub struct AskReply<S: Script> {
    //pub amount_offered: i32,
    //pub commodity: C,
    //pub offered_price: S,
    pub i_sold_them_for: S,
    pub quantity_sold: i32,
}
#[derive(Debug)]
pub struct BidReply<S: Script> {
    //pub amount_offered: i32,
    //pub commodity: C,
    //pub offered_price: S,
    pub sold_to_me_for: S, 
    pub quantity_sold: i32,
}


pub trait EERGAgent<C: Commodity, S: Script>
where for<'a> &'a S: Add<Output = S> + Div<Output = S> + Mul<Output = S> + Sub<Output = S>
 {
    fn get_average_historical_price<M: Market<C, S>>(&self, bazaar:&M, commodity_id: &C) -> Option<S>;

    fn determine_purchase_quantity<M: Market<C, S>>(&self, bazaar:&M, commodity: &C) -> i32;
    fn determine_sale_quantity<M: Market<C, S>>(&self, bazaar:&M, commodity_id: &C) -> i32;
    fn create_ask<M: Market<C, S>>(&self, bazaar:&M, commodity:&C, max_limit_to_sell: i32) -> Option<Ask<C, S>>;
    fn create_bid<M: Market<C, S>>(&self, bazaar:&M, commodity: &C, max_limit_to_buy: i32) -> Option<Bid<C, S>>;
    fn price_update_from_asks<M: Market<C, S>>(
        &mut self, 
        amount_put_up: i32,
        trade_price: S,
        replies: Vec<AskReply<S>>,
        commodity: &C, 
        market: &M,
        supply_cmp_demand: &std::cmp::Ordering,
        total_ammount_sold_this_round: i32
    );
    fn price_update_from_bids<M: Market<C, S>>(
        &mut self, 
        amount_wanted: i32,
        value_of_acquiring: S,
        replies: Vec<BidReply<S>>, 
        commodity: &C, 
        market: &M,
        supply_cmp_demand: &std::cmp::Ordering,
        total_ammount_sold_this_round: i32
    );
}

pub trait BaseValues<C: Commodity, S: std::cmp::PartialOrd>
{
    fn get_base_value(&self, commodity: &C) -> S;
    fn get_base_value_range(&self, commodity: &C) -> Range<S>;
}
impl<T: MarketAgentBasics<C, S>+ EERGAgentBasics<C, S>, C: Commodity, S: Script> EERGAgent<C, S> for T
where for<'a> &'a S: Add<Output = S> + Div<Output = S> + Mul<Output = S> + Sub<Output = S>,
Self: BaseValues<C, S>
{
    #[allow(dead_code)]
    fn get_average_historical_price<M: Market<C, S>>(&self, bazaar:&M, commodity_id: &C) -> Option<S>{
        bazaar.get_average_historical_price(commodity_id, self.get_lookback())
    }

    fn create_ask<M: Market<C, S>>(&self, bazaar:&M, commodity:&C, max_limit_to_sell: i32) -> Option<Ask<C, S>> {
		let ask_price = self.price_of(commodity);
		let ideal = self.determine_sale_quantity(bazaar, commodity);

		//can't sell less than limit
		let quantity_to_sell = ideal.min(max_limit_to_sell);
		if quantity_to_sell > 0 {
			return Some(Ask::new(*commodity, quantity_to_sell, ask_price))
		};
		return None;
	}
    fn create_bid<M: Market<C, S>>(&self, bazaar:&M, commodity: &C, max_limit_to_buy:i32) -> Option<Bid<C, S>> {
        let bid_price = self.price_of(commodity);
        let ideal = self.determine_purchase_quantity(bazaar, commodity);

        //can't buy more than limit
        let quantity_to_buy = ideal.min(max_limit_to_buy);
        if quantity_to_buy > 0 {
            return Some(Bid::new(*commodity, quantity_to_buy, bid_price));
        }
        None
    }
    fn determine_sale_quantity<M: Market<C, S>>(&self, bazaar:&M, commodity_id: &C) -> i32 {
        let Some(mean) = bazaar.get_average_historical_price(commodity_id, self.get_lookback()) else {
            logy!("error","No historical avg price");
            return 0
        };
        let Some(trading_range) = self.observe_trading_range(commodity_id) else {
            logy!("error", "no trading fange");
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
    fn price_update_from_asks<M: Market<C, S>>(
        &mut self, 
        amount_wanted: i32,
        offered_price: S,
        replies: Vec<AskReply<S>>, 
        commodity: &C, 
        market: &M,
        supply_cmp_demand: &std::cmp::Ordering,
        total_ammount_sold_this_round: i32,
    ) {
        todo!("{amount_wanted}{offered_price:?}{replies:?}{commodity:?}{market:?}{supply_cmp_demand:?},{total_ammount_sold_this_round}")
        /*
        let market_share = (quantity_sold as f32) / (total_ammount_of_commodit_sold_this_round as f32);
        let weight = (quantity_sold as f32) / (amount_put_up_for_sell as f32);
        let historic_mean_price = market
            .get_average_historical_price(commodity, self.get_lookback())
            .unwrap_or(S::ONE)
        ;
        let displacement = historic_mean_price * weight;
        let mut belief = self.get_price_beliefs(commodity).unwrap_or(Range::new(S::ONE, S::ONE));
        if quantity_sold == 0 {
            belief.shift(displacement * (-1.0 / 6.0));
        } else {
            for traded_price in i_sold_them_for{
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
        */
    }
    fn price_update_from_bids<M: Market<C, S>>(
        &mut self, 
        amount_put_up: i32,
        offered_price: S,
        replies: Vec<BidReply<S>>,
        commodity: &C, 
        market: &M,
        supply_cmp_demand: &std::cmp::Ordering,
        total_ammount_sold_this_round: i32
    ) {
        // calculating the values used
        let total_sold: i32 = replies
            .iter()
            .map(|x| x.quantity_sold)
            .sum();
        let percent_of_bid_fulfilled = amount_put_up as f64 / total_sold as f64;
        let mut belief = self.get_price_beliefs(commodity).unwrap_or(self.get_base_value_range(commodity));
        //let market_share = total_sold as f64 / total_ammount_sold_this_round as f64;
        let percent_of_inventory = self.current_inventory(commodity) as f64 / self.max_inventory_capacity(commodity) as f64;

        let mean_selling_price_maybe = if replies.is_empty() {
            None
        } else {
            let total_of_selling_prices: S = replies
                .iter()
                .fold(S::ZERO, |accum, x| &accum + &x.sold_to_me_for);
            let numbes_of_sales = replies.len();

            Some(total_of_selling_prices / numbes_of_sales as f32)
        };

        // now the actual logic of the algoryhtom
        if percent_of_bid_fulfilled >= 0.5 {
            let contract_amount = belief.max * 0.1;
            belief.contract(contract_amount);
        } else {
            let new_max = belief.max * 1.1;
            belief.set_max(new_max);
        }


        if total_sold != total_ammount_sold_this_round && percent_of_inventory < 0.25 {
            if let Some(mean_selling_price) = mean_selling_price_maybe {
                let displacement = (mean_selling_price - belief.mean()).abs();
                belief.shift(displacement);
            }
        } else if
            if let Some(mean_selling_price) = mean_selling_price_maybe {
                offered_price > mean_selling_price 
            } else {
                false
            } 
        // elseif offer price > trade price
        {
            let overbid = offered_price - mean_selling_price_maybe.expect("we aready tested that mean_selling_price_maybe was something when decide to go down");
            belief.shift(S::ZERO - (overbid * 1.1));
        } else if supply_cmp_demand == &Ordering::Greater 
            && if let Some(average_historical_price) = market.get_average_historical_price(commodity, self.get_lookback()) {
                    offered_price > average_historical_price 
                } else {
                    false
                }
        // elseif supply > demand and offer > historical mean price
        {
            let overbid = offered_price - market.get_average_historical_price(commodity, self.get_lookback()).expect("we aready tested that mean_selling_price_maybe was something when decide to go down");
            belief.shift(S::ZERO - (overbid * 1.1));
        } else 
        // elseif demand > supply
        //      then
        //          Translate belief range upwards by 1/5 historical mean price
        //      else
        //          Translate belief range downwards by 1/5 historical mean price
        {
            if let Some(average_historical_price) = market.get_average_historical_price(commodity, self.get_lookback()){
                if supply_cmp_demand != &Ordering::Less {
                    belief.shift(average_historical_price * 0.2);
                } else {
                    belief.shift(S::ZERO - (average_historical_price * 0.2));
                }

            }

        }
        self.set_price_beliefs(commodity, belief);
    }

}
