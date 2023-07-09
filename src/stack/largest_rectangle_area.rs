struct Rect{
    h:i32,
    x:i32,
}

pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut stack:Vec<Rect> = vec![];
    let mut max_area = 0;
    let length = heights.len() as i32;
    for (i, value) in heights.into_iter().enumerate() {
        let mut start = i as i32;
        while let Some(rect) = stack.pop() {
            if rect.h <= value {
                stack.push(rect);
                break;
            }
            max_area = std::cmp::max(max_area, rect.h * (i as i32 - rect.x));
            start = rect.x;
        }
        stack.push(Rect { h: value, x: start });
    }
    for rect in stack {
        max_area = std::cmp::max(max_area, rect.h * (length as i32 - rect.x))
    }
    return max_area;
}

mod tests {
    use super::*;

    #[test]
    fn case1() {
        // Given:
        let heights = vec![2, 1, 5, 6, 2, 3];

        // When:
        let result = largest_rectangle_area(heights);

        // Then:
        assert_eq!(result, 10);
    }

    #[test]
    fn case2() {
        // Given:
        let heights = vec![2, 4];

        // When:
        let result = largest_rectangle_area(heights);

        // Then:
        assert_eq!(result, 4);
    }
}
