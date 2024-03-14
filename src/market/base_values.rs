use crate::{market::Commodity, util::Range};

pub trait BaseValues<C: Commodity, S: std::cmp::PartialOrd>
{
    fn get_base_value(&self, commodity: &C) -> S;
    fn get_base_value_range(&self, commodity: &C) -> Range<S>;
}
