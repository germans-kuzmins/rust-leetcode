use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let hash_set: HashSet<i32> = nums.into_iter().collect();
        let mut max_size = 0;
        for i in hash_set.iter() {
            let prev = *i - 1;
            if !hash_set.contains(&prev) {
                let mut curr_seq_size = 1;
                let mut next = i + 1;

                while hash_set.contains(&next) {
                    curr_seq_size += 1;
                    next += 1;
                }
                if max_size < curr_seq_size {
                    max_size = curr_seq_size;
                }
            }
        }
        return max_size;
    }
}
