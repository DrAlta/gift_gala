use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

use super::util::Date;

use super::Script;

use crate::market::{Ask, Bid, Commodity, Market, MarketAgentBasics};
use crate::EERGAgent;

use crate::{AskReply, BidReply};

type AgentID = usize;
struct Askette<S: Script> {
    amount_put_up_for_sell: i32, 
    pub quantity: i32,
    trade_price: S,
    pub replies: Vec::<AskReply<S>>,
}
impl<S: Script> Askette<S>  {
    pub fn new(amount_put_up_for_sell: i32, trade_price: S) -> Self {
        Self { quantity: amount_put_up_for_sell.clone(), amount_put_up_for_sell, trade_price, replies: Vec::new() }
    }
    #[allow(dead_code)]
    pub fn trade_price(&self) -> &S {
        &self.trade_price
    }
    #[allow(dead_code)]
    pub fn amount_put_up_for_sell(&self) -> i32 {
        self.amount_put_up_for_sell
    }
}

struct Bidette<S: Script> {
    amount_wanted: i32, 
    pub quantity: i32,
    value_of_acquiring: S,
    pub replies: Vec::<BidReply<S>>,
}
impl<S: Script> Bidette<S>  {
    pub fn new(amount_wanted: i32, value_of_acquiring: S) -> Self {
        Self { quantity: amount_wanted.clone(), amount_wanted, value_of_acquiring, replies: Vec::new() }
    }
    #[allow(dead_code)]
    pub fn amount_wanted(&self) -> i32 {
        self.amount_wanted
    }
    #[allow(dead_code)]
    pub fn value_of_acquiring(&self) -> &S {
        &self.value_of_acquiring
    }

}

////////////////////////////////////////////////////////////////////////////



pub struct Bazaar<C: Commodity, S: Script, A: EERGAgent<C, S>, M: Market<C, S>> 
where for<'a> &'a S: Add<Output = S> + Div<Output = S> + Mul<Output = S> + Sub<Output = S>
{
    trading_agents: Vec<A>,
    traded_goods: Vec<C>,
    market: M,
    rust_dies_dedent_uses_in_the_def_of_other_types: std::marker::PhantomData<S>,
}

impl<C: Commodity, S: Script, A: EERGAgent<C, S>, M: Market<C, S>> Bazaar<C, S, A, M> 
where for<'a> &'a S: Add<Output = S> + Div<Output = S> + Mul<Output = S> + Sub<Output = S>
{
    pub fn new(trading_agents: Vec<A>, traded_goods: Vec<C>, market: M) -> Self {
        Bazaar{trading_agents, traded_goods, market, rust_dies_dedent_uses_in_the_def_of_other_types: PhantomData}
    }
    fn max_option(a:Option<S>, b:Option<S>) -> Option<S> {
        match(a, b) {
            (Some(x), None) => Some(x),
            (None, Some(x)) => Some(x),
            (Some(x), Some(y)) => if x > y {
                Some(x)
            } else {
                Some(y)
            },
            _ => None
        }
    
    }
}
/*
struct Offerette<S: Script> {
    pub agent_id: usize,
    pub quantity: i32,
    pub price: S
}
*/


//market sim
impl<C: Commodity, S: Script, A: EERGAgent<C, S> + MarketAgentBasics<C, S>, M: Market<C, S>> Bazaar<C, S, A, M> 
where for<'a> &'a S: Add<Output = S> + Div<Output = S> + Mul<Output = S> + Sub<Output = S>
{
    pub fn get_traded_goods(&self) ->  Vec<C> {
        self.traded_goods.clone()
    }

    fn produce(&mut self) {

    }
    pub fn sim_market (&mut self, count: i32) {
        self.produce();
        for i in 0..count {
            self.sim_market_step(i);
        }
    }

    pub fn sim_market_step (&mut self, date: Date) {
        for commodity in &self.get_traded_goods() {
            let mut askettes = HashMap::<AgentID, Askette<S>>::new();
            let mut bidettes = HashMap::<AgentID, Bidette<S>>::new();

            for i in 0..self.trading_agents.len() {
                if let Some(
                    Ask{
                        commodity:_, 
                        trade_price: price, 
                        amount_put_up: quantity
                    }
                ) 
                = 
                self.trading_agents[i].create_ask(
                    &self.market, 
                    &commodity, 
                    10
                ) {
                    askettes.insert(i, Askette::new(quantity, price));
                }
                if let Some(Bid{commodity:_, value_of_acquiring: price, amount_wanted: quantity})=self.trading_agents[i].create_bid(&self.market, &commodity, 10){
                    bidettes.insert(i, Bidette::new(quantity, price));
                }
            }//end gathering bids and asks

            Self::resolve_offers(&mut self.market, &mut self.trading_agents, commodity, askettes, bidettes, &date);
        } // end for commodity
    }   

    fn resolve_offers(
        market: &mut M, 
        trading_agents:&mut Vec<A>, 
        commodity: &C, 
        mut askettes:HashMap::<AgentID, Askette<S>>, 
        mut bidettes: HashMap::<AgentID, Bidette<S>>, 
        date: &Date
    ) {
        // we need to go through the bids in order so We'll creat Vec of the AgentIDs in tha correct order
        let mut bid_queue: Vec<AgentID> = bidettes.keys().map(|x| x.clone()).collect();

        bid_queue.sort_unstable_by(
            |a, b| 
                bidettes.get(a).unwrap().value_of_acquiring.partial_cmp(&bidettes.get(b).unwrap().value_of_acquiring).unwrap_or(std::cmp::Ordering::Equal)
        );
        // we need to go through the asks in order so We'll creat Vec of the AgentIDs in tha correct order
        let mut ask_queue: Vec<AgentID> = askettes.keys().map(|x| x.clone()).collect();

        ask_queue.sort_unstable_by(
            |a, b|
                askettes.get(b).unwrap().trade_price.partial_cmp(&askettes.get(a).unwrap().trade_price).unwrap_or(std::cmp::Ordering::Equal)
        );

        //let mut current_ask_reply: Option<(usize, AskReply<S>)> = None;
        //let mut current_bid_reply: Option<(usize, BidReply<S>)> = None;

        let mut pop_askettes = false;
        let mut pop_bidettes = false;
/*
        let mut total_amount_bought_from_agent_this_round = HashMap::new();
        let mut total_amount_sold_to_agent_this_round = HashMap::new();
*/
        let mut total_amount_sold_at_market_this_round = 0;
 
        let mut max_unmatched_bid: Option<S> = None;
        let mut max_unmatched_ask: Option<S> = None;
        let mut total_unbought = 0;
        let mut total_unmet_want = 0;
        /*
        let bidettes_size = bidettes.len() - 1;
        let askettes_size = askettes.len() - 1;
        */
        loop {
            // uses to tell if we are still processing asks
            if pop_askettes {
                pop_askettes = false;
                ask_queue.pop();
            }
            // uses to tell if we are still processing bits
            if pop_bidettes {
                pop_bidettes = false;
                bid_queue.pop();
            }

            if let Some(price) = max_unmatched_ask {
                market.push_max_unmatched_asks_history(*commodity, price, *date);
            }
            if let Some(price) = max_unmatched_bid {
                market.push_max_unmatched_bids_history(*commodity, price, *date);
            }
            let &buyer_id = bid_queue.last().unwrap();
            let &seller_id = ask_queue.last().unwrap();

            match (bidettes.get_mut(&buyer_id), askettes.get_mut(&seller_id)) {
                (Some(bidette), Some(askette)) => {
                    let quantity_traded = askette.quantity.min(bidette.quantity);
                    let clearing_price = askette.trade_price.average(&bidette.value_of_acquiring);

                    for quantity_actualy_traded in 0..(quantity_traded ) {
                        // first we try to get the money from the buyer
                        let Ok(_) = trading_agents
                            .get_mut(buyer_id.clone())
                            .expect("buyer agent not found")
                            .withdrawl(&clearing_price) 
                        else {
                                continue;
                        }; 
                        let seller_agent = trading_agents
                            .get_mut(seller_id.clone())
                            .expect("seller agent not found");
                        seller_agent.deposit(&clearing_price);
                    
                        let Ok(_) = seller_agent
                            .relinquish_good(commodity, &quantity_actualy_traded)
                        else {
                            continue;
                        };
                        askette.quantity -= quantity_actualy_traded;
                        if askette.quantity >= 0 {
                            pop_askettes = true;

                        }
                        total_amount_sold_at_market_this_round += quantity_actualy_traded;
                        //total_amount_bought_from_agent_this_round.add_or_insert(seller.agent_id, quantity_actualy_traded);
                        //total_amount_sold_to_agent_this_round.add_or_insert(seller.agent_id, quantity_actualy_traded);
                        bidette.quantity -= quantity_traded;
                        if bidette.quantity >= 0 {
                            pop_bidettes = true;
                        }


                        askette.replies.push(AskReply { 
                            i_sold_them_for: clearing_price.clone(), 
                            quantity_sold: quantity_actualy_traded, 
                        });
        
                        let buyer_agent = trading_agents
                            .get_mut(buyer_id.clone())
                            .expect("buyer agent not found");

                        buyer_agent.receive_good(commodity, &quantity_actualy_traded);

                        bidette.replies.push(
                            BidReply { 
                                sold_to_me_for: clearing_price.clone(), 
                                quantity_aquired: quantity_actualy_traded 
                            }
                        );
                            
                        break
                    }//end calculating quantity traded
                },
                (Some(Bidette{value_of_acquiring, quantity,..}), None) => {
                    max_unmatched_bid = Self::max_option(max_unmatched_bid, Some(*value_of_acquiring));
                    total_unbought += quantity.clone();                                      
                },
                (None, Some(Askette{trade_price, quantity, ..})) => {
                    max_unmatched_ask = Self::max_option(max_unmatched_ask, Some(*trade_price));
                    total_unmet_want += quantity.clone();
                },
                _ => break
            }
             
            assert!(total_unmet_want ==0 || total_unbought == 0)
        }
        let supply_cmp_demand: std::cmp::Ordering = match (max_unmatched_ask, max_unmatched_bid) {
            (Some(_), None) => std::cmp::Ordering::Greater,
            (None, Some(_)) => std::cmp::Ordering::Less,
            _ => std::cmp::Ordering::Equal
        };
        for (seller_id, Askette { amount_put_up_for_sell, quantity: _, trade_price, replies }) in askettes{
            trading_agents.get_mut(seller_id).unwrap()
                .price_update_from_asks::<M>(
                    amount_put_up_for_sell,
                    trade_price,
                    replies,
                    commodity,
                    market,
                    &supply_cmp_demand,
                    total_amount_sold_at_market_this_round.clone()
                )
            ;
        }
        for (buyer_id, Bidette { amount_wanted, quantity: _, value_of_acquiring, replies }) in bidettes{
            trading_agents.get_mut(buyer_id).unwrap()
                .price_update_from_bids::<M>(
                    amount_wanted,
                    value_of_acquiring,
                    replies,
                    commodity,
                    market,
                    &supply_cmp_demand,
                    total_amount_sold_at_market_this_round.clone()
                )
            ;
        }
    }
}

