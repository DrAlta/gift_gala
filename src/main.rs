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

fn main() {
    let mut bazaar = test_market::TestMarket::<MyCommodity>::def();
    bazaar.push_price_history(MyCommodity::Stuff, 1_f32, 1);
    bazaar.push_max_unfulfilled_bids_history(MyCommodity::Stuff, 1_f32, 2);
    let agent = TestAgent::<MyCommodity>::def();
    println!("{:?}", agent.determine_sale_quantity(&bazaar, &MyCommodity::Things));
    println!("{:?}", agent.determine_purchase_quantity(&bazaar, &MyCommodity::Things));

    println!("{:?}", agent.better_determine_sale_quantity(&bazaar, &MyCommodity::Things));
    println!("{:?}", agent.better_determine_purchase_quantity(&bazaar, &MyCommodity::Things));
}
