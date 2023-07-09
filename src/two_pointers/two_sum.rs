pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = numbers.len() - 1;
    while left < right {
        let sum = numbers[left] + numbers[right];
        match sum.cmp(&target) {
            std::cmp::Ordering::Equal => return vec![(left + 1) as i32, (right + 1) as i32],
            std::cmp::Ordering::Less => left += 1,
            std::cmp::Ordering::Greater => right -= 1,
        }
        
    }
    return vec![];
}

mod test {
    use super::*;

    #[test]
    fn case1() {
        // Given:
        let numbers = vec![2,7,11,15];
        let target = 9;

        // When:
        let result = two_sum(numbers, target);

        // Then:
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn case2() {
        // Given:
        let numbers = vec![2, 3, 4];
        let target = 6;

        // When:
        let result = two_sum(numbers, target);

        // Then:
        assert_eq!(result, vec![1, 3]);
    }

    #[test]
    fn case3() {
        // Given:
        let numbers = vec![-1, 0];
        let target = -1;

        // When:
        let result = two_sum(numbers, target);

        // Then:
        assert_eq!(result, vec![1, 2]);
    }
}
