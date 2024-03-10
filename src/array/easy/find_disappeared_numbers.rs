pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];

    let n = nums.len();

    for i in (1..n + 1) {
        if !nums.contains(&(i as i32)) {
            res.push(i as i32);
        }
    }

    res
}

#[cfg(test)]
pub mod tests_disappeared_number {
    use super::find_disappeared_numbers;

    #[test]
    pub fn test_dis_nr() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        find_disappeared_numbers(nums);
    }
}
