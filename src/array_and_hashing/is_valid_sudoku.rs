use std::collections::HashSet;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut visited = HashSet::new();
    for row in &board {
        visited.clear();
        for &item in row {
            if item != '.' && !visited.insert(item) {
                return false;
            };
        }
    }

    for col in 0..board[0].len() {
        visited.clear();
        for row in &board {
            if row[col] != '.' && !visited.insert(row[col]) {
                return false;
            };
        }
    }

    for start_row in (0..board.len()).step_by(3) {
        for start_col in (0..board.len()).step_by(3) {
            visited.clear();
            for i in start_row..start_row+3 {
                for j in start_col..start_col+3 {
                    if board[i][j] != '.' && !visited.insert(board[i][j]) {
                        return false;
                    };
                }
            }
        }
    }

    return true;
}

mod tests {
    use super::*;

    #[test]
    fn case1() {
        // Given:
        let input = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        // When:
        let mut result = is_valid_sudoku(input);

        // Then:
        assert_eq!(result, true);
    }

    #[test]
    fn case2() {
        // Given:
        let input = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        // When:
        let mut result = is_valid_sudoku(input);

        // Then:
        assert_eq!(result, false);
    }

    #[test]
    fn case3() {
        // Given:
        let input = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '8', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        // When:
        let mut result = is_valid_sudoku(input);

        // Then:
        assert_eq!(result, false);
    }
}
