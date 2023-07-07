use std::collections::HashSet;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; temperatures.len()];
        let mut stack: Vec<(i32, usize)> = vec![];
        for (i, v) in temperatures.into_iter().enumerate() {
            while let Some((last_temp, _)) = stack.last() {
                if *last_temp >= v {
                    break;
                }
                let (_, prev_index) = stack.pop().unwrap();
                result[prev_index] = i as i32 - prev_index as i32;
            }

            stack.push((v, i));
        }
        return result;
    }
}
