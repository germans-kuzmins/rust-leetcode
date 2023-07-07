pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; temperatures.len()];
        let mut stack:Vec<(i32, usize)> = vec![];
        for (i, v) in temperatures.into_iter().enumerate() {
            let last = stack.pop();
            match last {
                Some((last_temp, last_index)) => {
                    if last_temp < v {
                        
                    } else {
                        stack.push((last_temp, last_index));
                        stack.push((v, i));
                    }
                },
                None => stack.push((v, i))
            }
        }
        return vec![];
}

mod tests {
    use super::*;

    #[test]
    fn case1() {
        // Given:
        let temperatures = vec![73,74,75,71,69,72,76,73];

        // When:
        let result = daily_temperatures(temperatures);

        // Then:
        assert_eq!(result, vec![1,1,4,2,1,1,0,0]);
    }

    #[test]
    fn case2() {
        // Given:
        let temperatures = vec![30,40,50,60];

        // When:
        let result = daily_temperatures(temperatures);

        // Then:
        assert_eq!(result, vec![1,1,1,0]);
    }

    #[test]
    fn case3() {
        // Given:
        let temperatures = vec![30,40,50,60];

        // When:
        let result = daily_temperatures(temperatures);

        // Then:
        assert_eq!(result, vec![1,1,0]);
    }
}