use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let hash_set: HashSet<i32> = nums.into_iter().collect();
    let mut max_size = 0;
    for i in hash_set.iter() {
        let prev = *i - 1;
        if !hash_set.contains(&prev) {
            let mut curr_seq_size = 1;
            let mut next = i + 1;
           
            while hash_set.contains(&next) {
                curr_seq_size +=1;
                next +=1;
            }
            if max_size < curr_seq_size {
                max_size = curr_seq_size;
            }
        }
    }
    return max_size;
}

mod tests {
    use super::*;

    #[test]
    fn case1() {
        // Given:
        let nums = vec![100, 4, 200, 1, 3, 2];

        // When:
        let result = longest_consecutive(nums);

        // Then:
        assert_eq!(result, 4);
    }

    #[test]
    fn case2() {
        // Given:
        let nums = vec![0,3,7,2,5,8,4,6,0,1];

        // When:
        let result = longest_consecutive(nums);

        // Then:
        assert_eq!(result, 9);
    }
}
