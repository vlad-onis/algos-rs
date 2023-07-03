/*
Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place
such that each unique element appears only once. The relative order of the elements should be kept the same.
Then return the number of unique elements in nums.

https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/
*/

/// This solution beats: 32%, O(n^2)
#[allow(dead_code)]
pub struct Version1 {}

impl Version1 {
    #[allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut final_length: i32 = nums.len() as i32;

        for index in 0..nums.len() - 1 {
            if index >= (final_length - 1) as usize {
                break;
            }

            let searched_element = nums[index];
            let next_element = nums[index + 1];
            if searched_element == next_element {
                // If there is more than 1 duplicate, count how many there are
                let cnt = Version1::get_number_of_duplicates_from_current(
                    index + 1,
                    nums,
                    final_length as usize,
                );

                Version1::shift_left(index + 1, cnt as usize, nums);
                final_length -= cnt;
            }
        }

        final_length
    }

    pub fn get_number_of_duplicates_from_current(
        current_index: usize,
        nums: &mut [i32],
        length: usize,
    ) -> i32 {
        // This function is called when 1 duplicate is found and it is called with the index of the next element
        // it's purpose is to search for more consecutive duplicates
        let mut cnt = 1;

        for index in current_index..length - 1 {
            if nums[index + 1] == nums[current_index] {
                cnt += 1;
            } else {
                break;
            }
        }
        cnt
    }

    pub fn shift_left(current_index: usize, step: usize, nums: &mut Vec<i32>) {
        for index in current_index..(nums.len() - step) {
            nums[index] = nums[index + step];
        }
    }
}

/// This solution beats 59% in O(n)
#[allow(dead_code)]
pub struct Version2 {}

impl Version2 {
    #[allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut j = 0;
        for i in 1..nums.len() {
            if nums[i] != nums[j] {
                j += 1;
                nums[j] = nums[i];
            }
        }

        (j + 1) as i32
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_number_of_duplicates() {
        let mut nums: Vec<i32> = vec![1, 2, 3, 3, 3, 4, 5, 5, 6, 7];
        let length = nums.len();
        let cnt = Version1::get_number_of_duplicates_from_current(3, &mut nums, length);
        assert_eq!(cnt, 2);
    }

    #[test]
    fn test_shift_left_step() {
        let mut nums: Vec<i32> = vec![1, 2, 3, 3, 3, 4, 5, 5, 6, 7];
        let length = nums.len();
        let cnt = Version1::get_number_of_duplicates_from_current(3, &mut nums, length);
        Version1::shift_left(3, cnt as usize, &mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 5, 6, 7, 6, 7]);
    }

    #[test]
    fn test_shift_left_step_one() {
        let mut nums: Vec<i32> = vec![1, 2, 3, 3, 4, 5, 5, 6, 7];
        let length = nums.len();
        let cnt = Version1::get_number_of_duplicates_from_current(3, &mut nums, length);
        Version1::shift_left(3, cnt as usize, &mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 5, 6, 7, 7]);
    }

    #[test]
    fn test_duplicates_removal() {
        let mut nums: Vec<i32> = vec![1, 1, 2];
        let number_of_unique_elements = Version1::remove_duplicates(&mut nums);
        assert_eq!(number_of_unique_elements, 2);

        let mut nums: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let number_of_unique_elements = Version1::remove_duplicates(&mut nums);
        assert_eq!(number_of_unique_elements, 5);
    }
}
