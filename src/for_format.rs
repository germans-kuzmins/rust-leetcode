use std::collections::HashSet;

struct Rect {
    h: i32,
    x: i32,
}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;
        while left < right {
            let sum = &numbers[left] + &numbers[right];

            match sum.cmp(&target) {
                std::cmp::Ordering::Equal => return vec![(left + 1) as i32, (right + 1) as i32],
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Greater => right -= 1,
            }
        }
        return vec![];
    }
}
