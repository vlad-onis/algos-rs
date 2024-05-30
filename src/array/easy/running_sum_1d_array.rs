/// https://leetcode.com/problems/running-sum-of-1d-array/description/

pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 1..nums.len() {
        nums[i] = nums[i - 1] + nums[i];
    }

    nums
}
