use crate::Script;

use super::Commodity;

pub struct Bid<C: Commodity, S: Script> {
    pub amount_wanted: i32,
    pub commodity: C,
    pub value_of_acquiring: S
}
impl<C: Commodity, S: Script> Bid<C, S> {
    pub fn new(commodity: C, quantity: i32, price: S) -> Self {
        Bid{amount_wanted: quantity, commodity, value_of_acquiring: price}
    }
    pub fn strip(&self)-> (C, i32, S) {
        (self.commodity.clone(), self.amount_wanted.clone(), self.value_of_acquiring.clone())
    }
}
