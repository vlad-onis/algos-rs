fn shift_left(nums: &mut [i32]) {
    for i in 0..nums.len() - 1 {
        nums[i] = nums[i + 1];
    }
}

pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut i = 0;
    let mut zero_cnt = 0;
    while i < nums.len() {
        if nums[i] == 0 {
            zero_cnt += 1;
            shift_left(&mut nums[i..]);
        } else {
            i += 1;
        }
    }

    let starting_index = nums.len() - zero_cnt;
    nums[starting_index..].fill(0);
}

pub fn move_zeros_2_pointers(nums: &mut Vec<i32>) {
    let mut index = 0;
    let mut zero_searcher = 0;

    while index < nums.len() {
        if nums[index] != 0 {
            let temp = nums[zero_searcher];
            nums[zero_searcher] = nums[index];
            nums[index] = temp;

            index += 1;
            zero_searcher += 1;
        } else {
            index += 1;
        }
    }
}

#[cfg(test)]
pub mod move_zeros_tests {
    use super::*;

    #[test]
    pub fn test_move_zeros() {
        let mut nums = vec![1, 2, 3, 0, 4, 0, 5, 6, 7, 8];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7, 8, 0, 0]);

        let mut nums = vec![1, 2, 3, 0, 4, 0, 5, 6, 7, 8];
        move_zeros_2_pointers(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7, 8, 0, 0]);
    }

    #[test]
    pub fn test_leetcode1() {
        let mut nums = vec![0, 0, 1];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 0, 0]);

        let mut nums = vec![0, 0, 1];
        move_zeros_2_pointers(&mut nums);
        assert_eq!(nums, vec![1, 0, 0]);
    }
}
