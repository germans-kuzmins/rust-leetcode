use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for i in nums {
            *map.entry(i).or_insert(0) += 1;
        }


        // Convert the HashMap into a Vec of tuples
        let mut map_vec: Vec<(i32, i32)> = map.into_iter().collect();

        // Sort the Vec by the second element of each tuple (the value)
        map_vec.sort_by(|a, b| b.1.cmp(&a.1));

        return map_vec.iter().map(|&x| x.0).take(k as usize).collect();
    }
}
