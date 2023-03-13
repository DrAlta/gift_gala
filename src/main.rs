mod market;
mod test_agent;
mod test_market;
mod eerg;
mod util;
mod history_log;
use test_agent::TestAgent;
use eerg::EERGAgent as nope;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[allow(dead_code)]
pub enum MyCommodity {
    Stuff,
    Things,
}
impl market::Commodity for MyCommodity {

}

impl market::Script for f32{
    const ZERO: f32 = 0_f32;
    fn position_in_range(&self, min:&Self, max:&Self) -> f32 {
        (self - min) / (max - min)
    }
}

fn main() {
    let mut bazaar = test_market::TestMarket::<MyCommodity, f32>::def();
    bazaar.push_price_history(MyCommodity::Stuff, 1_f32, 1);
    bazaar.push_max_unfulfilled_bids_history(MyCommodity::Stuff, 1_f32, 2);
    let agent = TestAgent::<MyCommodity, f32>::def();
    println!("{:?}", agent.determine_sale_quantity(&bazaar, &MyCommodity::Things));
    println!("{:?}", agent.determine_purchase_quantity(&bazaar, &MyCommodity::Things));

    println!("{:?}", agent.better_determine_sale_quantity(&bazaar, &MyCommodity::Things));
    println!("{:?}", agent.better_determine_purchase_quantity(&bazaar, &MyCommodity::Things));
}
