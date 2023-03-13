mod market;
mod eerg_agent;
mod eerg_market;
mod util;
mod history_log;
use eerg_agent::EERGAgent;

fn main() {
    let mut bazaar = eerg_market::EERGMarket::def();
    bazaar.push_price_history(market::CommodityID::Stuff, 1_f32, 1);
    bazaar.push_max_unfulfilled_bids_history(market::CommodityID::Stuff, 1_f32, 2);
    let agent = market::EERGAgent::def();
    println!("{:?}", agent.determine_sale_quantity(&bazaar, &market::CommodityID::Things));
    println!("{:?}", agent.determine_purchase_quantity(&bazaar, &market::CommodityID::Things));

    println!("{:?}", agent.better_determine_sale_quantity(&bazaar, &market::CommodityID::Things));
    println!("{:?}", agent.better_determine_purchase_quantity(&bazaar, &market::CommodityID::Things));
}
