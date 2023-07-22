/// Given a sorted array of distinct integers and a target value, return the index if the target is found.
/// If not, return the index where it would be if it were inserted in order.
/// You must write an algorithm with O(log n) runtime complexity.

pub fn binary_find(nums: &Vec<i32>, mut left: usize, mut right: usize, target: i32) -> usize {
    while left <= right {
        let mid_index = (left + right) / 2;
        let mid_element = nums[mid_index];

        if target < mid_element {
            if mid_index == 0 {
                return 0;
            }
            right = mid_index - 1;
        } else if target > mid_element {
            left = mid_index + 1;
        } else {
            return mid_index;
        }
    }

    left
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> usize {
    if nums.is_empty() {
        return 0;
    }

    binary_find(&nums, 0, nums.len() - 1, target)
}

#[cfg(test)]
mod search_insert_position_tests {

    use super::*;

    #[test]
    pub fn test_empty_array() {
        let arr = vec![];
        let res = search_insert(arr, 5);
        assert_eq!(res, 0);
    }

    #[test]
    pub fn test_search_middle_element_existent() {
        let arr = vec![1, 2, 3, 4];
        let res = search_insert(arr, 3);
        assert_eq!(res, 2);
    }

    #[test]
    pub fn test_search_middle_element_nonexistent() {
        let arr = vec![1, 2, 4, 5];
        let res = search_insert(arr, 3);
        assert_eq!(res, 2);
    }

    #[test]
    pub fn test_search_first_element() {
        let arr = vec![1, 2, 4, 5];
        let res = search_insert(arr, 1);
        assert_eq!(res, 0);
    }

    #[test]
    pub fn test_search_last_element() {
        let arr = vec![1, 2, 4, 5];
        let res = search_insert(arr, 5);
        assert_eq!(res, 3);
    }

    #[test]
    pub fn test_search_first_element_nonexistent() {
        let arr = vec![1, 2, 4, 5];
        let res = search_insert(arr, -1);
        assert_eq!(res, 0);
    }

    #[test]
    pub fn test_search_last_element_nonexistent() {
        let arr = vec![1, 2, 4, 5];
        let res = search_insert(arr, 6);
        assert_eq!(res, 4);
    }
}
