/// https://leetcode.com/problems/k-radius-subarray-averages/description/
use std::iter;

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

pub fn get_averages2(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let len = 2 * k as u64 + 1;

    let mut result = vec![-1; nums.len()];
    if nums.len() < len as usize {
        return result;
    }

    let mut sum = nums[..len as usize]
        .into_iter()
        .map(|&num| num as u64)
        .sum::<u64>();
    result[k] = (sum / len) as i32;

    for i in k + 1..nums.len() - k {
        sum += nums[i + k] as u64;
        sum -= nums[i - k - 1] as u64;
        result[i] = (sum / len) as i32;
    }

    result
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
}
