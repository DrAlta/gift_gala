use crate::Script;

use super::Commodity;

#[derive(Debug)]
pub struct Ask<C: Commodity, S: Script> {
    pub amount_put_up: i32,
    pub commodity: C,
    pub trade_price: S
}
impl<C: Commodity, S: Script> Ask<C, S> {
    pub fn new(commodity: C, quantity: i32, price: S) -> Self {
        Ask{amount_put_up: quantity, commodity, trade_price: price}
    }
    pub fn strip(&self)-> (C, i32, S) {
        (self.commodity.clone(), self.amount_put_up.clone(), self.trade_price.clone())
    }
}
