use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut duplicates = HashSet::with_capacity(nums.len());
    for i in &nums {
        if duplicates.contains(i) {
            return true;
        }
        duplicates.insert(*i);
    }
    return false;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
    }

    #[test]
    fn case2() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
    }

    #[test]
    fn case3() {
        assert_eq!(
            contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        );
    }
}
