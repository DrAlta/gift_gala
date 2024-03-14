use crate::Script;

#[derive(Debug)]
pub struct BidReply<S: Script> {
    //pub amount_offered: i32,
    //pub commodity: C,
    //pub offered_price: S,
    pub sold_to_me_for: S, 
    pub quantity_aquired: i32,
}

