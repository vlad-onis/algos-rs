// https://leetcode.com/problems/array-partition/description/

pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    nums.iter().step_by(2).sum()
}

#[cfg(test)]
pub mod array_pair_sum_tests {
    use super::*;

    #[test]
    pub fn test_leetcode1() {
        let nums = vec![1, 2, 3, 4];
        let res = array_pair_sum(nums);

        assert_eq!(res, 4);
    }
}
