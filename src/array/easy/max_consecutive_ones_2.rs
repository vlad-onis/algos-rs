/// Given a binary array nums, return the maximum number of consecutive
/// 1's in the array if you can flip at most one 0.

/// https://leetcode.com/problems/max-consecutive-ones-ii/

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut right = 0;
    let mut left = 0;
    let mut max_sequence = 0;
    let mut zero_count = 0;
    while right < nums.len() {
        if nums[right] == 0 {
            zero_count += 1;
        }

        while zero_count == 2 {
            if nums[left] == 0 {
                zero_count -= 1;
            }
            left += 1;
        }
        let window_size = right - left + 1;
        max_sequence = max_sequence.max(window_size);
        right += 1;
    }

    max_sequence as i32
}

#[cfg(test)]
pub mod max_consecutive_ones_tests {
    use super::*;

    #[test]
    fn test_leetcode1() {
        let nums = vec![1, 0, 1, 1, 0];
        assert_eq!(find_max_consecutive_ones(nums), 4);
    }

    #[test]
    fn test_leetcode2() {
        let nums = vec![1, 0, 1, 1, 0, 1];
        assert_eq!(find_max_consecutive_ones(nums), 4);
    }

    #[test]
    fn test_leetcode3() {
        let nums = vec![1, 1, 0, 1];
        assert_eq!(find_max_consecutive_ones(nums), 4);
    }
}
