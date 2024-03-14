pub mod market;
use market::{Commodity, Market};
mod test_agent;
mod test_market;

mod eerg;
pub use eerg::{AskReply, BidReply, EERGAgent, EERGAgentBasics};
//mod eerg;
mod util;
mod history_log;
use test_agent::TestAgent;


mod bot_bazaar;

mod script;
pub use script::Script;

type Float = f32;

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

impl Script for Float{
    const ONE: Float = 1.0;
    const TWO: Float = 2.0;
    const ZERO: Float = 0.0;
    fn average(&self, other: &Self) -> Self {
        (self + other) / 2.0
    }
    fn position_in_range(&self, min:&Self, max:&Self) -> Float {
        (self - min) / (max - min)
    }
    fn difference(&self, other: &Self) -> Self {
        (self - other).abs()
    }
    fn abs(&self) -> Self {
        Float::abs(*self)
    }
}

fn main() {
    let mut market = test_market::TestMarket::<MyCommodity, Float>::def();
    market.push_price_history(MyCommodity::Stuff, 1.0, 1);
    market.push_max_unmatched_bids_history(MyCommodity::Stuff, 1.0, 2);
    let agent = TestAgent::<MyCommodity, Float>::def();

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

