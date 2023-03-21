pub fn max_area(height: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = height.len() - 1;
    let mut max_area = 0;
    while i != j {
        let left = height[i];
        let right = height[j];
        let min = left.min(right);
        let distance = (j - i) as i32;
        max_area = (min * distance).max(max_area);
        if left > right {
            j = j - 1;
        } else {
            i = i + 1;
        }
    }
    return max_area;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn example2() {
        assert_eq!(max_area(vec![1, 1]), 1);
    }
}
