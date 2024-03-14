use crate::Script;

#[derive(Debug)]
pub struct AskReply<S: Script> {
    //pub amount_offered: i32,
    //pub commodity: C,
    //pub offered_price: S,
    pub i_sold_them_for: S,
    pub quantity_sold: i32,
}
