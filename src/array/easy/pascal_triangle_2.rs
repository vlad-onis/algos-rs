/// Given an integer rowIndex, return the rowIndexth (0-indexed) row of the Pascal's triangle.
/// https://leetcode.com/problems/pascals-triangle-ii/

pub fn compute_next_row(current_row: &Vec<i32>, current_index: usize) -> Vec<i32> {
    let mut next_row: Vec<i32> = vec![0; current_index + 1];
    let size = next_row.len();

    next_row[0] = 1;
    next_row[size - 1] = 1;

    for i in 0..current_row.len() - 1 {
        let computed_element = current_row[i] + current_row[i + 1];
        next_row[i + 1] = computed_element;
    }

    next_row
}

pub fn get_row(row_index: i32) -> Vec<i32> {
    let first_row = vec![1];
    let second_row = vec![1, 1];

    if row_index == 0 {
        return first_row;
    } else if row_index == 1 {
        return second_row;
    }

    let mut next_row = vec![];
    let mut current_row = second_row;
    for i in 2..(row_index + 1) {
        next_row = compute_next_row(&current_row, i as usize);
        current_row = next_row.clone();
    }

    next_row
}

#[cfg(test)]
mod pascal_triangle_2_tests {

    use super::*;

    #[test]
    pub fn test_next_row_generation() {
        let current_row = vec![1, 1];
        let next_row = compute_next_row(&current_row, 2);
        assert_eq!(next_row, vec![1, 2, 1]);

        let current_row = vec![1, 5, 10, 10, 5, 1];
        let next_row = compute_next_row(&current_row, 6);
        assert_eq!(next_row, vec![1, 6, 15, 20, 15, 6, 1]);
    }

    #[test]
    pub fn test_get_triangle_row() {
        let row = 3;
        let next_row = get_row(row);

        assert_eq!(next_row, vec![1, 3, 3, 1]);
    }
}
