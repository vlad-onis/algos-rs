use core::num;

/// Given an integer numRows, return the first numRows of Pascal's triangle.
/// https://leetcode.com/problems/pascals-triangle/
///

pub fn compute_next_row(current_row: &Vec<i32>, current_index: usize) -> Vec<i32> {
    let mut next_row: Vec<i32> = vec![0; current_index];
    let size = next_row.len();

    next_row[0] = 1;
    next_row[size - 1] = 1;

    for i in 0..current_row.len() - 1 {
        let computed_element = current_row[i] + current_row[i + 1];
        next_row[i + 1] = computed_element;
    }

    next_row
}

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    if num_rows == 0 {
        return vec![];
    } else if num_rows == 1 {
        return vec![vec![1]];
    } else if num_rows == 2 {
        return vec![vec![1], vec![1, 1]];
    }

    let mut res = vec![vec![1], vec![1, 1]];
    let mut current_row = vec![1, 1];
    for i in 3..num_rows + 1 {
        let next_row = compute_next_row(&current_row, i as usize);
        current_row = next_row.clone();
        res.push(next_row);
    }

    res
}

#[cfg(test)]
pub mod pascal_triangle_tests {

    use super::*;

    #[test]
    pub fn test_next_row_generation() {
        let current_row = vec![1, 1];
        let next_row = compute_next_row(&current_row, 3);

        assert_eq!(next_row, vec![1, 2, 1]);

        let current_row = vec![1, 5, 10, 10, 5, 1];
        let next_row = compute_next_row(&current_row, 7);
        assert_eq!(next_row, vec![1, 6, 15, 20, 15, 6, 1]);
    }

    #[test]
    pub fn test_zero_one_and_two() {
        let num_rows = 0;
        let res = generate(num_rows);
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(res, empty);

        let num_rows = 1;
        let res = generate(num_rows);
        assert_eq!(res, vec![vec![1]]);

        let num_rows = 2;
        let res = generate(num_rows);
        assert_eq!(res, vec![vec![1], vec![1, 1]]);
    }

    #[test]
    pub fn test_triangle_of_3() {
        let num_rows = 3;
        let res = generate(num_rows);
        assert_eq!(res, vec![vec![1], vec![1, 1], vec![1, 2, 1]]);
    }

    #[test]
    pub fn test_triangle_of_5() {
        let num_rows = 5;
        let res = generate(num_rows);
        assert_eq!(
            res,
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
}
