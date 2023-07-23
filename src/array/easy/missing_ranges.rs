/// You are given an inclusive range [lower, upper] and a sorted unique integer array nums,
/// where all elements are within the inclusive range.
/// A number x is considered missing if x is in the range [lower, upper] and x is not in nums.
/// Return the shortest sorted list of ranges that exactly covers all the missing numbers.
/// That is, no element of nums is included in any of the ranges, and each missing number is covered by one of the ranges.

/// https://leetcode.com/problems/missing-ranges

pub fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<Vec<i32>> {
    if nums.is_empty() {
        return vec![vec![lower, upper]];
    }

    let mut ranges: Vec<Vec<i32>> = vec![];

    if nums[0] != lower {
        ranges.push(vec![lower, nums[0] - 1]);
    }

    for i in 0..nums.len() - 1 {
        if nums[i + 1] - nums[i] > 1 {
            ranges.push(vec![nums[i] + 1, nums[i + 1] - 1]);
        }
    }

    if nums[nums.len() - 1] != upper {
        ranges.push(vec![nums[nums.len() - 1] + 1, upper]);
    }

    ranges
}

#[cfg(test)]
pub mod missing_ranges_tests {
    use super::*;

    #[test]
    pub fn leetcode_tests() {
        let nums = vec![0, 1, 3, 50, 75];
        let lower = 0;
        let upper = 99;
        let res = find_missing_ranges(nums, lower, upper);

        assert_eq!(
            res,
            vec![vec![2, 2], vec![4, 49], vec![51, 74], vec![76, 99]]
        );
    }
}
