/// Given an integer array nums, move all the even integers at the beginning of the array followed by all the odd integers.
/// Return any array that satisfies this condition.
/// https://leetcode.com/problems/sort-array-by-parity/

pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;

    while i < nums.len() {
        if nums[i] % 2 == 0 {
            let mut temp = nums[i];
            nums[i] = nums[j];
            nums[j] = temp;
            i += 1;
            j += 1;
        } else {
            i += 1;
        }
    }

    return nums;
}
