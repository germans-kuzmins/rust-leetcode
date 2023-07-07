use std::collections::HashSet;

impl Solution {
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
}
