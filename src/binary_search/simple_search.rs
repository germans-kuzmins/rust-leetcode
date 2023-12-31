pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut l, mut r) = (0, nums.len());

    while l < r {
        let m = l + (r - l) / 2;
        match target.cmp(&nums[m]) {
            std::cmp::Ordering::Equal => return m as i32,
            std::cmp::Ordering::Less => r = m,
            std::cmp::Ordering::Greater => l = m + 1,
        }
    }

    return -1;
}

mod test {
    #[test]
    fn case1() {
        // Given:
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;

        // When:
        let result = super::search(nums, target);

        // Then:
        assert_eq!(result, 4);
    }

    #[test]
    fn case2() {
        // Given:
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;

        // When:
        let result = super::search(nums, target);

        // Then:
        assert_eq!(result, -1);
    }

    #[test]
    fn case3() {
        // Given:
        let nums = vec![-1];
        let target = -1;

        // When:
        let result = super::search(nums, target);

        // Then:
        assert_eq!(result, 0);
    }
}