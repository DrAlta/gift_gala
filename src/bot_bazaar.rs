use std::marker::PhantomData;

use super::util::Date;

use super::market::{Ask, Bid, Commodity, Market, Script, MarketAgentBasics};
use super::eerg::EERGAgent;

struct OfferReply<C: Commodity, S: Script> {
    pub agent_id : usize,
    pub amount_offered: i32,
    pub good: C,
    pub offered_price: S,
    pub sold_for: Vec<S>,
    pub quantity_sold: i32,
}


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
        for good in &self.get_traded_goods() {
            let mut askettes = Vec::<Offerette<S>>::new();
            let mut bidettes = Vec::<Offerette<S>>::new();
            for i in 0..self.trading_agents.len() {
                if let Some(Ask{good:_, price, quantity})=self.trading_agents[i].create_ask(&self.market, &good, 10){
                    askettes.push(Offerette { agent_id: i, quantity, price});
                }
                if let Some(Bid{good:_, price, quantity})=self.trading_agents[i].create_bid(&self.market, &good, 10){
                    bidettes.push(Offerette { agent_id: i, quantity, price});
                }
            }//end gathering bids and asks

            Self::resolve_offers(&mut self.market, &mut self.trading_agents, good, askettes, bidettes, &date);
        } // end for good
    }   

    fn resolve_offers(market: &mut M, trading_agents:&mut Vec<A>, good: &C, mut askettes:Vec::<Offerette<S>>, mut bidettes:Vec::<Offerette<S>>, date: &Date) {
        //We are going to be poping things off the end so these need to be in the oposite order from the pap

        bidettes.sort_unstable_by(
            |a, b| 
                a.price.partial_cmp(&b.price).unwrap_or(std::cmp::Ordering::Equal)
        );
        askettes.sort_unstable_by(
            |a, b|
                b.price.partial_cmp(&a.price).unwrap_or(std::cmp::Ordering::Equal)
        );

        let mut current_ask_reply: Option<OfferReply<C, S>> = None;
        let mut current_bid_reply: Option<OfferReply<C, S>> = None;
        let mut ask_replies = Vec::<OfferReply<C, S>>::new();
        let mut bid_replies = Vec::<OfferReply<C, S>>::new();

        let mut pop_askettes = false;
        let mut pop_bidettes = false;

        let mut number_of_buyers = 0;
        let mut number_of_sellers = 0;

        let mut max_unfulfilled_bid: Option<S> = None;
        let mut max_unfulfilled_ask: Option<S> = None;

        let bidettes_size = bidettes.len() - 1;
        let askettes_size = askettes.len() - 1;
        loop {
            if pop_askettes {
                pop_askettes = false;
                askettes.pop();
                if let Some(reply) = current_ask_reply{
                    if reply.quantity_sold > 0 {
                        number_of_buyers += 1;
                    }
                    ask_replies.push(reply);
                }
                current_ask_reply = None;
            }
            if pop_bidettes {
                pop_bidettes = false;
                bidettes.pop();
                if let Some(reply) = current_bid_reply {
                    if reply.quantity_sold > 0 {
                        number_of_sellers += 1;
                    }
                    bid_replies.push(reply);
                }
                current_bid_reply = None;
            }

            if let Some(price) = max_unfulfilled_ask {
                market.push_max_unfulfilled_asks_history(*good, price, *date);
            }
            if let Some(price) = max_unfulfilled_bid {
                market.push_max_unfulfilled_bids_history(*good, price, *date);
            }

            match (bidettes.get_mut(bidettes_size), askettes.get_mut(askettes_size)) {
                (Some(buyer), Some(seller)) => {
                    let quantity_traded = seller.quantity.min(buyer.quantity);
                    let clearing_price = seller.price.average(&buyer.price);

                    for quantity_actualy_traded in 0..(quantity_traded ) {
                        if let Ok(_) = trading_agents
                            .get_mut(buyer.agent_id)
                            .expect("buyer agent not found")
                            .withdrawl(&clearing_price) {
                                trading_agents
                                    .get_mut(seller.agent_id)
                                    .expect("seller agent not found")
                                    .deposit(&clearing_price)
                        } else {
                            let seller_agent = trading_agents
                            .get_mut(seller.agent_id)
                            .expect("seller agent not found");
                            if let Ok(_) = seller_agent
                                .relinquish_good(good, &quantity_actualy_traded) {
                                    seller.quantity -= quantity_actualy_traded;
                                    if seller.quantity >= 0 {
                                        pop_askettes = true;

                                    }
                                    buyer.quantity -= quantity_traded;
                                    if buyer.quantity >= 0 {
                                        pop_bidettes = true;
                                    }

                                    //update reply for the seller
                                    let mut reply_to_seller = current_ask_reply.unwrap_or(
                                        OfferReply{
                                            agent_id: seller.agent_id.clone(),
                                            good: good.clone(),
                                            quantity_sold: 0,
                                            sold_for: Vec::new(),
                                            amount_offered: seller.quantity.clone(),
                                            offered_price: seller.price.clone(),
                                        }
                                    );

                                    reply_to_seller.quantity_sold += quantity_actualy_traded;
                                    reply_to_seller.sold_for.push(clearing_price.clone());
                    
                                    current_ask_reply = Some(reply_to_seller);
                    
                    
                                    let buyer_agent = trading_agents
                                        .get_mut(buyer.agent_id)
                                        .expect("buyer agent not found");

                                    buyer_agent.receive_good(good, &quantity_actualy_traded);
                                
                                    //update reply for the bid
                                    let mut reply_to_buyer = current_bid_reply.unwrap_or(
                                        OfferReply{
                                            agent_id: buyer.agent_id.clone(),
                                            good: good.clone(),
                                            quantity_sold: 0,
                                            sold_for: Vec::new(),
                                            amount_offered: buyer.quantity.clone(),
                                            offered_price: buyer.price.clone(),
                                        }
                                    );
                                    reply_to_buyer.quantity_sold += quantity_actualy_traded;
                                    reply_to_buyer.sold_for.push(clearing_price.clone());
                    
                                    current_bid_reply = Some(reply_to_buyer);
                    
                                }
                            break
                        }
                    }//end callc quantity traded
                },
                (Some(Offerette{price, quantity, agent_id}), None) => {
                    max_unfulfilled_bid = Self::max_option(max_unfulfilled_bid, Some(*price));
                    bid_replies.push(OfferReply{
                        agent_id: agent_id.clone(),
                        good: good.clone(),
                        quantity_sold: 0,
                        sold_for: Vec::new(),
                        amount_offered: quantity.clone(),
                        offered_price: price.clone(),
                    });
                },
                (None, Some(Offerette{price, quantity, agent_id})) => {
                    max_unfulfilled_ask = Self::max_option(max_unfulfilled_ask, Some(*price));
                    ask_replies.push(OfferReply{
                        agent_id: agent_id.clone(),
                        good: good.clone(),
                        quantity_sold: 0,
                        sold_for: Vec::new(),
                        amount_offered: quantity.clone(),
                        offered_price: price.clone(),
                    });

                },
                _ => break
            }
        }
        for ask in ask_replies{
            trading_agents.get_mut(ask.agent_id).unwrap()
                .price_update_from_ask::<M>(
                    ask.amount_offered,
                    good,
                    ask.offered_price,
                    ask.sold_for,
                    ask.quantity_sold,
                    &(number_of_buyers > 1)
                )
            ;
        }
    }
}

