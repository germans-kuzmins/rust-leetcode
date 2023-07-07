pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut zipped: Vec<(i32, i32)> = position.into_iter().zip(speed.into_iter()).collect();
    zipped.sort_by_key(|&(key, _)| std::cmp::Reverse(key));
    let mut stack: Vec<(i32, i32)> = vec![];
    for car in zipped {
        let arrive_time = (target - car.0) as f32 / car.1 as f32;

        if let Some(prev_car) = stack.last() {
            let prev_car_arrive_time = (target - prev_car.0) as f32 / prev_car.1 as f32;
            if arrive_time > prev_car_arrive_time {
                stack.push(car);
            }
        } else {
            stack.push(car)
        }
    }

    return stack.len() as i32;
}

mod tests {
    use super::*;

    #[test]
    fn case1() {
        // Given:
        let target = 12;
        let position = vec![10, 8, 0, 5, 3];
        let speed = vec![2, 4, 1, 1, 3];

        // When:
        let result = car_fleet(target, position, speed);

        // Then:
        assert_eq!(result, 3);
    }

    #[test]
    fn case2() {
        // Given:
        let target = 10;
        let position = vec![3];
        let speed = vec![3];

        // When:
        let result = car_fleet(target, position, speed);

        // Then:
        assert_eq!(result, 1);
    }

    #[test]
    fn case3() {
        // Given:
        let target = 100;
        let position = vec![0, 2, 4];
        let speed = vec![4, 2, 1];

        // When:
        let result = car_fleet(target, position, speed);

        // Then:
        assert_eq!(result, 1);
    }

    #[test]
    fn case4() {
        // Given:
        let target = 10;
        let position = vec![0, 4, 2];
        let speed = vec![2, 1, 3];

        // When:
        let result = car_fleet(target, position, speed);

        // Then:
        assert_eq!(result, 1);
    }
}
