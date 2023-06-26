pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut lookup = std::collections::HashMap::new();

    for (i, v) in nums.iter().enumerate() {
        let found = lookup.get(&(target - v));
        if found.is_some() {
            return vec![*found.unwrap(), i as i32];
        }

        lookup.insert(v, i as i32);
    }
    return vec![0, 0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(two_sum(vec![2,7,11,15], 9), vec![0, 1]);
    }

    #[test]
    fn case2() {
        assert_eq!(two_sum(vec![3,2,4], 6), vec![1, 2]);
    }

    #[test]
    fn case3() {
        assert_eq!(two_sum(vec![3,3], 6), vec![0, 1]);
    }
}
