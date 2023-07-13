pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }

    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut left = 0;
    let mut right = (rows * cols);

    while left < right {
        let m = left + (right - left) / 2;
        let value = matrix[m / cols][m % cols];
        match target.cmp(&value) {
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Less => right = m,
            std::cmp::Ordering::Greater => left = m + 1,
        }
    }
    return false;
}

mod test {
    use super::*;

    #[test]
    fn test_search_matrix_existing_element() {
        // Given
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 3;

        // When
        let result = search_matrix(matrix, target);

        // Then
        assert_eq!(result, true);
    }

    #[test]
    fn test_search_matrix_non_existing_element() {
        // Given
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 13;

        // When
        let result = search_matrix(matrix, target);

        // Then
        assert_eq!(result, false);
    }

    #[test]
    fn test_search_matrix_empty_matrix() {
        // Given
        let matrix: Vec<Vec<i32>> = Vec::new();
        let target = 5;

        // When
        let result = search_matrix(matrix, target);

        // Then
        assert_eq!(result, false);
    }

    #[test]
    fn test_search_matrix_empty_row() {
        // Given
        let matrix = vec![Vec::new()];
        let target = 5;

        // When
        let result = search_matrix(matrix, target);

        // Then
        assert_eq!(result, false);
    }

    #[test]
    fn test_search_matrix_single_value() {
        // Given
        let matrix = vec![vec![1]];
        let target = 1;

        // When
        let result = search_matrix(matrix, target);

        // Then
        assert_eq!(result, true);
    }
}
