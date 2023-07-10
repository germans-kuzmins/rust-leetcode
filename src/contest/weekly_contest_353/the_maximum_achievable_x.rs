pub fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
    return num + t * 2;
}

mod test {
    use super::*;

    #[test]
    fn case1() {
        // Given:
        let num = 4;
        let t = 1;
        
        // When:
        let result = the_maximum_achievable_x(num, t);

        // Then:
        assert_eq!(result, 6)
    }

    fn case2() {
        // Given:
        let num = 3;
        let t = 2;
        
        // When:
        let result = the_maximum_achievable_x(num, t);

        // Then:
        assert_eq!(result, 7)
    }
}
