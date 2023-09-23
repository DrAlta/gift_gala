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
    pub fn get_values(&self, commodity:&K) -> Option<&Vec<T>> {
        self.values.get(commodity)
    }
    pub fn push(&mut self, commodity: K, value: T, date: Date) {
        let Some(values) = self.values.get_mut(&commodity) else {
            self.values.insert(commodity.clone(), vec!(value));
            if let Some(_) = self.dates.insert(commodity, vec!(date)) {
                panic!("their wasn't any values but there was dates, values[commodity] and dates[commodity] should be the same langth");
            }
            return;
        };

        values.push(value);

        self.dates.get_mut(&commodity)
            .expect("their was values but there wasn't any dates, values[commodity] and dates[commodity] should be the same langth")
            .push(date);
    }
}
