/// Given an integer array nums, rotate the array to the right by k steps, where k is non-negative.
/// https://leetcode.com/problems/rotate-array

pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    if nums.len() < 2 {
        return;
    }

    let mut dummy: Vec<i32> = Vec::new();
    nums.into_iter().for_each(|element| {
        dummy.push(*element);
    });

    let size = nums.len();

    for index in 0..size {
        nums[(index + k as usize) % size] = dummy[index];
    }
}

pub fn reverse_array(nums: &mut Vec<i32>, mut start: usize, mut end: usize) {
    while start < end {
        let temp = nums[start];
        nums[start] = nums[end];
        nums[end] = temp;
        start += 1;
        end -= 1;
    }
}

pub fn rotate_in_place(nums: &mut Vec<i32>, k: i32) {
    let k = k as usize % nums.len();
    if nums.len() < 2 {
        return;
    }

    if k == 0 {
        return;
    }

    reverse_array(nums, 0, nums.len() - 1);
    reverse_array(nums, 0, (k - 1) as usize);
    reverse_array(nums, k as usize, nums.len() - 1);
}

#[cfg(test)]
pub mod rotate_array_tests {
    use super::*;

    #[test]
    fn rotate_cases() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn rotate_in_place_cases() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        rotate_in_place(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn rotate_reverse_array() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let size = nums.len();
        reverse_array(&mut nums, 0, size - 1);
        assert_eq!(nums, vec![7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn rotate_reverse_part_of_array() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let size = nums.len();
        reverse_array(&mut nums, 0, 2);
        reverse_array(&mut nums, 3, size - 1);
        assert_eq!(nums, vec![3, 2, 1, 7, 6, 5, 4]);
    }
}
