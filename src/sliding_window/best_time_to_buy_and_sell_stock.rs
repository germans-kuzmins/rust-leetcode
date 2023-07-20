// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut result = 0;

    let mut left = 0;
    let mut right = 1;
    while right < prices.len() {
        
        if prices[left] < prices[right] {
            let profit: i32 = prices[right] - prices[left];
            result = result.max(profit);
        } else {
            left = right;
        }

        right +=1;
    }

    return result;
}

mod test {
    use super::*;

    #[test]
    fn case1() {
        // Given:
        let prices = vec![7, 1, 5, 3, 6, 4];

        // When:
        let result = max_profit(prices);

        // Then:
        assert_eq!(result, 5);
    }

    #[test]
    fn case2() {
        // Given:
        let prices = vec![7, 6, 4, 3, 1];

        // When:
        let result = max_profit(prices);

        // Then:
        assert_eq!(result, 0);
    }

    #[test]
    fn case3() {
        // Given:
        let prices = vec![4];

        // When:
        let result = max_profit(prices);

        // Then:
        assert_eq!(result, 0);
    }

    #[test]
    fn case4() {
        // Given:
        let prices = vec![1,4,2];

        // When:
        let result = max_profit(prices);

        // Then:
        assert_eq!(result, 3);
    }

    #[test]
    fn case5() {
        // Given:
        let prices = vec![2,1,2,1,0,1,2];

        // When:
        let result = max_profit(prices);

        // Then:
        assert_eq!(result, 2);
    }
}
