mod market;
mod test_agent;
mod test_market;
mod eerg;
mod util;
mod history_log;
use test_agent::TestAgent;
use eerg::EERGAgent as nope;

use crate::market::{Commodity, Market};

mod bot_bazaar;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum MyCommodity {
    Stuff,
    Things,
}
impl market::Commodity for MyCommodity {
    fn into_vec() -> Vec<Self> {
    vec!(MyCommodity::Stuff, MyCommodity::Things)
    }
}

impl market::Script for f32{
    const ONE: f32 = 1_f32;
    const ZERO: f32 = 0_f32;
    fn average(&self, other: &Self) -> Self {
        (self + other) / 2_f32
    }
    fn position_in_range(&self, min:&Self, max:&Self) -> f32 {
        (self - min) / (max - min)
    }
}

fn main() {
    let mut market = test_market::TestMarket::<MyCommodity, f32>::def();
    market.push_price_history(MyCommodity::Stuff, 1_f32, 1);
    market.push_max_unfulfilled_bids_history(MyCommodity::Stuff, 1_f32, 2);
    let agent = TestAgent::<MyCommodity, f32>::def();

    println!("{:?}", agent.determine_sale_quantity(&market, &MyCommodity::Things));
    println!("{:?}", agent.determine_purchase_quantity(&market, &MyCommodity::Things));
    if let Some(thing) = agent.create_bid(&market, &MyCommodity::Things, 1) {
        println!("bid: {:?}", thing.strip());
    }
    if let Some(thing) = agent.create_ask(&market, &MyCommodity::Things, 1) {
        println!("ask: {:?}", thing.strip());
    }

    println!("{:?}", agent.better_determine_sale_quantity(&market, &MyCommodity::Things));
    println!("{:?}", agent.better_determine_purchase_quantity(&market, &MyCommodity::Things));

    let mut bazaar = bot_bazaar::Bazaar::new(vec!(agent),MyCommodity::into_vec(), market);
    bazaar.sim_market(1);
}

