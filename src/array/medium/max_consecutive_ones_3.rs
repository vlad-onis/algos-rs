use std::cmp::max;

/// https://leetcode.com/problems/max-consecutive-ones-iii/description/

pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut left = 0;
    let mut right = 0;
    let mut max_seq = 0;
    let mut flips = 0;

    if nums.is_empty() {
        return 0;
    }

    if k >= nums.iter().filter(|e| **e == 0).count() as i32 {
        return nums.len() as i32;
    }

    while right < nums.len() {
        let el = nums[right];

        if el == 1 {
            right += 1;
        } else if el == 0 && flips < k {
            right += 1;
            flips += 1;
        } else if el == 0 && flips >= k {
            if nums[left] == 0 && flips > 0 {
                flips -= 1;
            }

            left += 1;
        }

        if left > right {
            right = left;
        }

        let seq = right - left;
        max_seq = max(max_seq, seq);
    }
    max_seq as i32
}

#[cfg(test)]
pub mod tests_max_consecutive_ones_3 {

    use crate::array::easy::longest_common_prefix::longest_common_prefix;

    use super::*;

    #[test]
    pub fn test_empty_arr() {
        let arr = vec![];
        let max = longest_ones(arr, 2);

        assert_eq!(max, 0);
    }

    #[test]
    pub fn test_k_bigger() {
        let arr = vec![1, 1, 0, 0, 0, 0];
        let k = 5;
        let max = longest_ones(arr, k);

        assert_eq!(max, 6);
    }

    #[test]
    pub fn test_leetcode1() {
        let arr = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
        let k = 2;

        let max_ones = longest_ones(arr, k);
        assert_eq!(max_ones, 6);
    }

    #[test]
    pub fn test_leetcode2() {
        let arr = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
        let k = 3;
        let max_ones = longest_ones(arr, k);

        assert_eq!(max_ones, 10);
    }

    #[test]
    pub fn test_leetcode3() {
        let arr = vec![0, 0, 1, 1, 1, 0, 0];
        let k = 0;
        let max_ones = longest_ones(arr, k);

        assert_eq!(max_ones, 3);
    }
}
