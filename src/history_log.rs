use std::collections::HashMap;

use super::util::Date;
pub struct HistoryLog<K, T> {
    values: HashMap<K, Vec<T>>,
    dates: HashMap<K, Vec<Date>>,
}
impl<K: std::hash::Hash + Eq + Clone, T> HistoryLog<K, T> {
    pub fn def() -> Self {
        HistoryLog{values: HashMap::new(), dates: HashMap::new()}
    }
    pub fn get_values(&self, good:&K) -> Option<&Vec<T>> {
        self.values.get(good)
    }
    pub fn push(&mut self, good: K, value: T, date: Date) {
        let Some(values) = self.values.get_mut(&good) else {
            self.values.insert(good.clone(), vec!(value));
            if let Some(_) = self.dates.insert(good, vec!(date)) {
                panic!("their wasn't any values but there was dates, values[good] and dates[good] should be the same langth");
            }
            return;
        };

        values.push(value);

        self.dates.get_mut(&good)
            .expect("their was values but there wasn't any dates, values[good] and dates[good] should be the same langth")
            .push(date);
    }
}
