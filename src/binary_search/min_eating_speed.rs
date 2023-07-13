//https://leetcode.com/problems/koko-eating-bananas/
pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let max = piles.iter().max().unwrap();

    let mut left = 1;
    let mut right = *max + 1;
    let mut result = i32::MAX;

    while left < right {
        let m = left + (right - left) / 2;
        let value: i32 = piles
            .iter()
            .map(|x| {
                let quotient = x / m;
                let remainder = x % m;
                if remainder > 0 {
                    quotient + 1
                } else {
                    quotient
                }
            })
            .sum();
        match value.cmp(&h) {
            std::cmp::Ordering::Equal | std::cmp::Ordering::Less => {
                result = result.min(m);
                right = m;
            }
            std::cmp::Ordering::Greater => left = m + 1,
        }
    }

    return result;
}

mod tests {
    use super::*;

    #[test]
    fn case1() {
        // Given
        let piles = vec![3, 6, 7, 11];
        let h = 8;

        // When
        let result = min_eating_speed(piles, h);

        // Then
        assert_eq!(result, 4);
    }

    #[test]
    fn case2() {
        // Given
        let piles = vec![30, 11, 23, 4, 20];
        let h = 5;

        // When
        let result = min_eating_speed(piles, h);

        // Then
        assert_eq!(result, 30);
    }

    #[test]
    fn case3() {
        // Given
        let piles = vec![30, 11, 23, 4, 20];
        let h = 6;

        // When
        let result = min_eating_speed(piles, h);

        // Then
        assert_eq!(result, 23);
    }
}
