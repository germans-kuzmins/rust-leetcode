pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut answer: Vec<Vec<i32>> = Vec::new();
    let mut nums = nums;
    nums.sort_unstable();
    for first in 0..nums.len() - 2 {
        if first > 0 && nums[first] == nums[first - 1] {
            continue;
        }
        let (mut second, mut third) = (first + 1, nums.len() - 1);
        while second < third {
            let sum = nums[first] + nums[second] + nums[third];
            if 0 == sum {
                answer.push(vec![nums[first], nums[second], nums[third]]);
            }

            if sum < 0 {
                let second_value = nums[second];
                while {
                    second += 1;
                    second < third && nums[second] == second_value
                } {}
            } else {
                let third_value = nums[third];
                while {
                    third -= 1;
                    third > second && nums[third] == third_value
                } {}
            }
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        // Given:
        let nums = vec![-1, 0, 1, 2, -1, -4];

        // When:
        let result = three_sum(nums);

        // Then:
        let expected_result: Vec<Vec<i32>> = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn case2() {
        // Given:
        let nums = vec![-1, 0, 1, 2, -1, -4];

        // When:
        let result = three_sum(nums);

        // Then:
        let expected_result: Vec<Vec<i32>> = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn case3() {
        // Given:
        let nums = vec![0, 0, 0, 0];

        // When:
        let result = three_sum(nums);

        // Then:
        let expected_result: Vec<Vec<i32>> = vec![vec![0, 0, 0]];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn case4() {
        // Given:
        let nums = vec![-2, 0, 1, 1, 2];

        // When:
        let result = three_sum(nums);

        // Then:
        let expected_result: Vec<Vec<i32>> = vec![vec![-2, 0, 2], vec![-2, 1, 1]];
        assert_eq!(result, expected_result);
    }
}
