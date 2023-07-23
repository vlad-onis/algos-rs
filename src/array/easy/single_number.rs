use std::collections::HashMap;

/// Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
/// You must implement a solution with a linear runtime complexity and use only constant extra space.
/// https://leetcode.com/problems/single-number/

pub fn single_number_with_map(nums: Vec<i32>) -> i32 {
    let mut fqy_map: HashMap<i32, i32> = HashMap::new();

    for index in 0..nums.len() {
        *fqy_map.entry(nums[index]).or_default() += 1;
    }

    for key in fqy_map.keys() {
        if fqy_map[key] == 1 {
            return *key;
        }
    }

    0
}

pub fn single_number_xor(nums: Vec<i32>) -> i32 {
    let mut a = 0;

    for element in nums {
        a = a ^ element;
    }

    return a;
}

#[cfg(test)]
pub mod single_number_tests {

    use super::*;

    #[test]
    pub fn test_array() {
        let nums = vec![1, 2, 3, 1, 2];
        let res = single_number_with_map(nums);
        assert_eq!(res, 3);
    }

    #[test]
    pub fn test_xor() {
        let nums = vec![1, 2, 3, 1, 2];
        let res = single_number_xor(nums);
        assert_eq!(res, 3);
    }
}
