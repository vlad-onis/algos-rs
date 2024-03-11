use std::iter;

pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
    let mut seq: Vec<i32> = iter::repeat(0).take(nums.len()).collect();

    nums.into_iter()
        .enumerate()
        .for_each(|(x, y)| seq[(y - 1) as usize] += 1);

    let res = seq
        .into_iter()
        .enumerate()
        .filter(|(x, y)| *y == 0)
        .map(|(x, y)| (x + 1) as i32)
        .collect();

    res
}

#[cfg(test)]
pub mod tests_disappeared_number {
    use super::find_disappeared_numbers;

    #[test]
    pub fn test_dis_nr() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        let res = find_disappeared_numbers(nums);
        assert_eq!(res, vec![5, 6]);

        let nums = vec![1, 1];
        let res = find_disappeared_numbers(nums);
        assert_eq!(res, vec![2]);
    }
}
