use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for i in nums {
        *map.entry(i).or_insert(0) += 1;
    }

    // Convert the HashMap into a Vec of tuples
    let mut map_vec: Vec<(i32, i32)> = map.into_iter().collect();

    // Sort the Vec by the second element of each tuple (the value)
    map_vec.sort_by(|a: &(i32, i32), b| b.1.cmp(&a.1));

    return map_vec.iter().map(|&x| x.0).take(k as usize).collect();
}

mod tests {
    use super::top_k_frequent;

    #[test]
    fn case1() {
        // Given:
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;

        // When:
        let mut result = top_k_frequent(nums, k);
        result.sort();

        // Then:
        let expected = vec![1, 2];
        assert_eq!(result, expected);
    }

    #[test]
    fn case2() {
        // Given:
        let nums = vec![1];
        let k = 1;

        // When:
        let result = top_k_frequent(nums, k);

        // Then:
        let expected = vec![1];
        assert_eq!(result, expected);
    }

    #[test]
    fn case3() {
        // Given:
        let nums = vec![1, 2];
        let k = 2;

        // When:
        let mut result = top_k_frequent(nums, k);
        result.sort();

        // Then:
        let expected = vec![1, 2];
        assert_eq!(result, expected);
    }
}
