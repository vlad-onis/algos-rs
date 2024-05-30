/// https://leetcode.com/problems/minimum-value-to-get-positive-step-by-step-sum/description/

pub fn min_start_value(nums: Vec<i32>) -> i32 {
    let mut start_value = 0;
    let mut sum = 0;

    for num in nums {
        sum += num;
        if sum < 1 {
            start_value += sum.abs() + 1;
            sum = 1;
        }
    }

    if start_value == 0 {
        1
    } else {
        start_value
    }
}

pub mod tests_minimum_value_to_get_positive_step_by_step_sum {

    use super::*;

    #[test]
    pub fn test_empty_array() {
        let arr = vec![];
        let s = min_start_value(arr);
        assert_eq!(s, 0);
    }

    #[test]
    pub fn test_leetcode1() {
        let arr = vec![-3, 2, -3, 4, 2];
        let s = min_start_value(arr);
        assert_eq!(s, 5);
    }
}
