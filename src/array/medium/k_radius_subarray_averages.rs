use core::num;
/// https://leetcode.com/problems/k-radius-subarray-averages/description/
use std::iter;

use crate::array::easy::max_average_subarray_1::get_avg;

pub fn get_averages_prefix_sum(nums: Vec<i32>, k: i32) -> Vec<i32> {
    if nums.is_empty() {
        return vec![];
    }

    // build prefix sum
    let mut prefix_sum: Vec<i64> = iter::repeat(0).take(nums.len()).collect();
    prefix_sum[0] = 0;
    for i in 1..nums.len() {
        prefix_sum[i] = prefix_sum[i - 1] + nums[i] as i64;
    }

    let mut results = Vec::new();

    for i in 0..nums.len() {
        let k = k as usize;

        if i.checked_sub(k).is_none() || i.checked_add(k).is_none() || i + k >= nums.len() {
            results.push(-1);
            continue;
        }

        let sum_of_elements_in_k_radius =
            prefix_sum[i + k] - prefix_sum[i - k] + nums[i - k] as i64;

        let number_of_elements_in_k_radius = k * 2 + 1;
        let avg = sum_of_elements_in_k_radius / number_of_elements_in_k_radius as i64;
        results.push(avg as i32)
    }

    results
}

pub fn find_avg(nums: &Vec<i32>, index: usize, k: usize) -> i32 {
    let mut sum = 0;

    for i in index - k..index + k {
        sum += nums[i];
    }

    return sum / (2 * k as i32 + 1);
}

pub fn get_averages_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let window_size = (k as usize * 2 + 1) as i32;
    let mut res: Vec<i32> = iter::repeat(-1).take(nums.len()).collect();

    let k = k as usize;

    if k == 0 {
        return nums;
    }

    if 2 * k + 1 > nums.len() {
        return res;
    }

    let mut window_sum = 0;

    // Compute the sum of the first window
    for i in 0..window_size as usize {
        window_sum += nums[i] as i64;
    }

    // set the first average
    res[k] = (window_sum / window_size as i64) as i32;

    for i in window_size as usize..nums.len() {
        window_sum = window_sum + nums[i] as i64 - nums[i - window_size as usize] as i64;
        let avg = (window_sum / window_size as i64) as i32;
        res[i - k] = avg;
    }

    res
}

#[cfg(test)]
pub mod tests_get_averages {

    use super::*;

    #[test]
    pub fn test_empty_arr() {
        let arr = vec![];
        let res = get_averages_prefix_sum(arr, 2);
        assert_eq!(res, vec![]);
    }

    #[test]
    pub fn test_leetcode() {
        // 100.000
        let arr: Vec<i32> = iter::repeat(100000).take(100000).collect();
        let res = get_averages_prefix_sum(arr, 4000);
    }

    #[test]
    pub fn test_sliding_window() {
        let arr: Vec<i32> = vec![2, 2, 1];
        let res = get_averages_sliding_window(arr, 1);
        println!("{:?}", res);
    }
}
