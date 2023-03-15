use std::marker::PhantomData;

use super::util::Date;

use super::market::{Ask, Bid, Commodity, Market, Script, MarketAgentBasics};
use super::eerg::EERGAgent;
pub struct Bazaar<C: Commodity, S: Script, A: EERGAgent<C, S>, M: Market<C, S>> {
    trading_agents: Vec<A>,
    traded_goods: Vec<C>,
    market: M,
    rust_dies_dedent_uses_in_the_def_of_other_types: std::marker::PhantomData<S>,
}

impl<C: Commodity, S: Script, A: EERGAgent<C, S>, M: Market<C, S>> Bazaar<C, S, A, M> {
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
struct Offerette<S: Script> {
    pub agent_id: usize,
    pub quantity: i32,
    pub price: S
}


//market sim
impl<C: Commodity, S: Script, A: EERGAgent<C, S> + MarketAgentBasics<C, S>, M: Market<C, S>> Bazaar<C, S, A, M> {

    fn produce(&mut self) {

    }
    pub fn sim_market (&mut self, count: i32) {
        self.produce();
        for i in 0..count {
            self.sim_market_step(i);
        }
    }

    pub fn sim_market_step (&mut self, date: Date) {
        let mut askettes = Vec::<Offerette<S>>::new();
        let mut bidettes = Vec::<Offerette<S>>::new();
        for good in &self.traded_goods {
            for i in 0..self.trading_agents.len() {
                if let Some(Ask{good:_, price, quantity})=self.trading_agents[i].create_ask(&self.market, &good, 10){
                    askettes.push(Offerette { agent_id: i, quantity, price});
                }
                if let Some(Bid{good:_, price, quantity})=self.trading_agents[i].create_bid(&self.market, &good, 10){
                    bidettes.push(Offerette { agent_id: i, quantity, price});
                }
            }//end gathering bids and asks

            //We are going to be poping things off the end so these need to be in the oposite order from the pap

            bidettes.sort_unstable_by(
                |a, b| 
                    a.price.partial_cmp(&b.price).unwrap_or(std::cmp::Ordering::Equal)
            );
            askettes.sort_unstable_by(
                |a, b|
                    b.price.partial_cmp(&a.price).unwrap_or(std::cmp::Ordering::Equal)
            );
            let mut pop_askettes = false;
            let mut pop_bidettes = false;


            let mut max_unfulfilled_bid: Option<S> = None;
            let mut max_unfulfilled_ask: Option<S> = None;

            let bidettes_size = bidettes.len() - 1;
            let askettes_size = askettes.len() - 1;

            match (bidettes.get_mut(bidettes_size), askettes.get_mut(askettes_size)) {
                (Some(buyer), Some(seller)) => {
                    let quantity_traded = seller.quantity.min(buyer.quantity);
                    let clearing_price = seller.price.average(&buyer.price);

                    for i in 0..(quantity_traded ) {
                        if let Ok(_) = self.trading_agents
                            .get_mut(buyer.agent_id)
                            .expect("buyer agent not found")
                            .withdrawl(&clearing_price) {
                                self.trading_agents
                                    .get_mut(seller.agent_id)
                                    .expect("seller agent not found")
                                    .deposit(&clearing_price)
                        } else {
                            if let Ok(_) = self.trading_agents
                                .get_mut(seller.agent_id)
                                .expect("seller agent not found")
                                .relinquish_good(good, &i) {
                                    seller.quantity -= quantity_traded;
                                    if seller.quantity >= 0 {
                                        pop_askettes = true;
                                    }
                                    buyer.quantity -= quantity_traded;
                                    if buyer.quantity >= 0 {
                                        pop_bidettes = true;
                                    }
                                    self.trading_agents
                                        .get_mut(buyer.agent_id)
                                        .expect("buyer agent not found")
                                        .receive_good(good, &i)
                            }
                            break
                        }
                    }
                },
                (Some(Offerette{price, quantity:_, agent_id: _}), None) => max_unfulfilled_bid = Self::max_option(max_unfulfilled_bid, Some(*price)),
                (None, Some(Offerette{price, quantity:_, agent_id: _})) => max_unfulfilled_ask = Self::max_option(max_unfulfilled_ask, Some(*price)),
                _ => ()
            }

            if pop_askettes {
                askettes.pop();
            }
            if pop_bidettes {
                bidettes.pop();
            }

            if let Some(price) = max_unfulfilled_ask {
                self.market.push_max_unfulfilled_asks_history(*good, price, date);
            }
            if let Some(price) = max_unfulfilled_bid {
                self.market.push_max_unfulfilled_bids_history(*good, price, date);
            }
        } // end for good
    }   
}
