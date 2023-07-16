// https://leetcode.com/problems/time-based-key-value-store/
use std::collections::HashMap;

struct TimeMap {
    values: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            values: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.values
            .entry(key)
            .or_insert(Vec::new())
            .push((timestamp, value))
    }

    fn get(&self, key: String, timestamp: i32) -> String {

        if let Some(found_vec) = self.values.get(&key) {

            return match found_vec.binary_search_by_key(&timestamp, |&(t, _)| t) {
                Ok(i)  => found_vec[i].1.clone(),
                Err(i) => if i > 0 { found_vec[i-1].1.clone() } else { String::from("") }
            }
        }

        String::from("")
    }
}

mod test {
    use super::*;

    #[test]
    fn case1() {
        // Given:
        let mut time_map = TimeMap::new();

        // When:
        time_map.set("foo".to_string(), "bar".to_string(), 1);

        // Then:
        assert_eq!(time_map.get("foo".to_string(), 1), "bar".to_string());
        assert_eq!(time_map.get("foo".to_string(), 3), "bar".to_string());

        // When:
        time_map.set("foo".to_string(), "bar2".to_string(), 4);

        // Then:
        assert_eq!(time_map.get("foo".to_string(), 4), "bar2".to_string());
        assert_eq!(time_map.get("foo".to_string(), 5), "bar2".to_string());
    }

    #[test]
    fn case2() {
        // Given:
        let mut time_map = TimeMap::new();

        // When:
        time_map.set("love".to_string(), "high".to_string(), 10);
        time_map.set("love".to_string(), "low".to_string(), 20);

        // Then:
        assert_eq!(time_map.get("love".to_string(), 5), "".to_string());
        assert_eq!(time_map.get("love".to_string(), 10), "high".to_string());
        assert_eq!(time_map.get("love".to_string(), 15), "high".to_string());
        assert_eq!(time_map.get("love".to_string(), 20), "low".to_string());
        assert_eq!(time_map.get("love".to_string(), 25), "low".to_string());
    }
}
