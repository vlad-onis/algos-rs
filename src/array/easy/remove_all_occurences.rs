/// Given an integer array nums and an integer val, remove all occurrences of val in nums in-place.
/// The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.

/// Uses in-build methods from vec to solve the problem
#[allow(dead_code)]
pub fn remove_element_retain(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|element| *element != val);
    nums.len() as i32
}

#[allow(dead_code)]
pub fn remove_element_pointers(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut from_start_index: usize = 0;
    let mut from_end_index = (nums.len() - 1) as i32;

    while from_start_index <= from_end_index as usize {
        if nums[from_start_index] == val {
            // Not nice but if we treat from_end_index as usize
            // There are cornercases that could trigger a subtraction
            // with overflow
            // e.g. [3,3] and val = 3
            // So we made from_end_index as i32 and the algorithm holds
            if from_end_index == 0 {
                from_end_index = -1;
                break;
            }

            nums.swap(from_start_index, from_end_index as usize);
            from_end_index -= 1;
        } else {
            from_start_index += 1;
        }
    }
    (from_end_index + 1) as i32
}

#[cfg(test)]
pub mod remove_all_occurences_tests {

    use super::*;

    #[test]
    pub fn test_empty_array() {
        let mut nums: Vec<i32> = vec![];

        let res = remove_element_pointers(&mut nums, 0);
        assert_eq!(res, 0);
    }

    #[test]
    pub fn test_one_element_same() {
        let mut nums: Vec<i32> = vec![1];

        let res = remove_element_pointers(&mut nums, 1);
        assert_eq!(res, 0);
    }

    #[test]
    pub fn test_one_element_different() {
        let mut nums: Vec<i32> = vec![1];

        let res = remove_element_pointers(&mut nums, 2);
        assert_eq!(res, 1);
    }

    #[test]
    pub fn test_two_element_same() {
        let mut nums: Vec<i32> = vec![1, 1];

        let res = remove_element_pointers(&mut nums, 1);
        assert_eq!(res, 0);
    }
}
