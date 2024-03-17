/// The pivot index is the index where the sum of all the numbers strictly to the left of
/// the index is equal to the sum of all the numbers strictly to the index's right.

/// If the index is on the left edge of the array, then the left sum is 0
/// because there are no elements to the left. This also applies to
/// the right edge of the array.

/// Return the leftmost pivot index. If no such index exists, return -1.

pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let sum: i32 = nums.iter().sum();
    let mut left_sum = sum;
    let mut right_sum = 0;

    let mut index = 0;
    let last = nums.len() - 1;
    let mut right_index = last;

    while index <= last {
        right_sum += nums[last - index];
        index += 1;

        if index > last {
            break;
        }

        right_index = last - index;

        left_sum = sum - right_sum - nums[right_index];

        if right_sum == left_sum {
            return right_index as i32;
        }
    }

    -1
}

#[cfg(test)]
pub mod tests_pivot_index {
    use super::pivot_index;

    #[test]
    pub fn test_pivot_leetcode1() {
        let nums = vec![1, 7, 3, 6, 5, 6];
        let res = pivot_index(nums);

        assert_eq!(res, 3);
    }

    #[test]
    pub fn test_pivot_leetcode2() {
        let nums = vec![1, 2, 3];
        let res = pivot_index(nums);

        assert_eq!(res, -1);
    }

    #[test]
    pub fn test_pivot_leetcode3() {
        let nums = vec![2, 1, -1];
        let res = pivot_index(nums);

        assert_eq!(res, 0);
    }
}
