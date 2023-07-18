use core::num;

/// Given an integer array nums sorted in non-decreasing order, remove some duplicates in-place such that each unique element
/// appears at most twice. The relative order of the elements should be kept the same.
/// Since it is impossible to change the length of the array in some languages,
/// you must instead have the result be placed in the first part of the array nums. More formally,
/// if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result.
/// It does not matter what you leave beyond the first k elements.
/// Return k after placing the final result in the first k slots of nums.
/// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii

/// Shifts elements to the left
/// Used to mimic removal of elements.
fn move_elements_to_the_left(numbers: &mut Vec<i32>, start_index: usize, offset: usize) {
    for i in start_index..numbers.len() - offset {
        numbers[i] = numbers[i + offset];
    }
}

/// Verifies if the array is composed of only 1 element repeated throughout the array
fn array_has_only_1_number(numbers: &mut Vec<i32>) -> bool {
    let cnt = numbers
        .iter()
        .filter(|element| **element != numbers[0])
        .count();

    cnt == 0
}

#[allow(dead_code)]
pub fn remove_duplicates_from_sorted_array_in_place(numbers: &mut Vec<i32>) -> i32 {
    if numbers.is_empty() {
        return 0;
    }

    if numbers.len() <= 2 {
        return numbers.len() as i32;
    }

    // return 2 because there are for sure at least 2 elements
    // in the array at this point and they are the same
    if array_has_only_1_number(numbers) {
        return 2;
    }

    // setup initial state
    let mut index = 0;
    let mut array_size = numbers.len();
    let mut count_same_element = 1;
    let mut start_index = 0;

    while index < array_size - 1 {
        if numbers[index] != numbers[index + 1] {
            if count_same_element > 2 {
                move_elements_to_the_left(numbers, start_index, count_same_element - 2);

                // reset the index after the shift and set the correct array_size
                // according to the identical element count
                index = start_index - 1;
                array_size = array_size - count_same_element + 2;
            }
            count_same_element = 1;
        } else {
            // count identical elements and if 3 are found
            // we set the index where the shift left will begin
            count_same_element += 1;
            if count_same_element == 3 {
                start_index = index + 1;
            }
        }

        index += 1;
    }

    // our loop fails to take care of the last group of identical elements
    // So we treat that outside the loop adjusting the array size according to
    // their count
    if count_same_element >= 3 {
        array_size = array_size - count_same_element + 2;
    }

    array_size as i32
}

#[cfg(test)]
pub mod remove_duplicates_sorted_tests {

    use super::*;

    #[test]
    fn test_array_has_only_1_number() {
        let mut numbers: Vec<i32> = vec![1, 1, 1];
        assert!(array_has_only_1_number(&mut numbers));
    }

    #[test]
    fn test_move_elements_to_left() {
        let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        // res = [1,2,3,6,7,8,9]
        move_elements_to_the_left(&mut numbers, 3, 2);
        assert_eq!(numbers, vec![1, 2, 3, 6, 7, 8, 9, 8, 9]);

        let mut numbers: Vec<i32> = vec![1, 1, 1, 1, 1, 1, 2];
        // res = [1,1,2,1,1,1,2]
        let cnt = 6;
        let start_index = 2;
        let offset = cnt - 2;
        move_elements_to_the_left(&mut numbers, start_index, offset);
        assert_eq!(numbers, vec![1, 1, 2, 1, 1, 1, 2]);

        let mut numbers: Vec<i32> = vec![1, 1, 2, 2, 2, 2, 2, 3, 4, 5];
        let cnt = 5;
        let start_index = 4;
        let offset = cnt - 2;
        // res = [1,2,2,3,4,5,3,4,5]
        move_elements_to_the_left(&mut numbers, start_index, offset);
        assert_eq!(numbers, vec![1, 1, 2, 2, 3, 4, 5, 3, 4, 5]);
    }

    #[test]
    fn test_empty_array() {
        let mut numbers: Vec<i32> = vec![];
        let res = remove_duplicates_from_sorted_array_in_place(&mut numbers);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_sorted_array() {
        let mut numbers: Vec<i32> = vec![1, 1, 1, 1, 1, 1, 1, 2];
        let res = remove_duplicates_from_sorted_array_in_place(&mut numbers);
        assert_eq!(res, 3);

        let mut numbers: Vec<i32> = vec![1, 1, 1, 2, 2, 3];
        let res = remove_duplicates_from_sorted_array_in_place(&mut numbers);
        assert_eq!(res, 5);
    }

    #[test]
    fn test_leetcode_case() {
        let mut numbers: Vec<i32> = vec![1, 1, 1];
        let res = remove_duplicates_from_sorted_array_in_place(&mut numbers);
        assert_eq!(res, 2);

        let mut numbers: Vec<i32> = vec![1, 1, 1, 1];
        let res = remove_duplicates_from_sorted_array_in_place(&mut numbers);
        assert_eq!(res, 2);

        let mut numbers: Vec<i32> = vec![1, 1, 1, 2, 2, 2, 3, 3];
        let res = remove_duplicates_from_sorted_array_in_place(&mut numbers);
        assert_eq!(res, 6);

        let mut numbers: Vec<i32> = vec![-3, -1, -1, 0, 0, 0, 0, 0];
        let res = remove_duplicates_from_sorted_array_in_place(&mut numbers);
        assert_eq!(res, 5);
    }
}
