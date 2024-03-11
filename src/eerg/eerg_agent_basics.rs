use crate::{market::Commodity, util::Range, Script};
// Emergent Economies for Role Playing Games Agent
pub trait EERGAgentBasics<C: Commodity, S: Script> {
    fn get_price_beliefs(&self, commodity:&C) -> Option<Range<S>>;
    fn price_of(&self, commodity: &C) -> S ;
    fn set_price_beliefs(&mut self, commodity:&C, belief: Range<S>);
}