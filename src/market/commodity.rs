pub trait Commodity: std::fmt::Debug + std::hash::Hash + Eq + Copy {
    fn into_vec() -> Vec<Self>;
}