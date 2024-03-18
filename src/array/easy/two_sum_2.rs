// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/description/

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut start = 0;
    let mut end = numbers.len() - 1;

    while start < end {
        if numbers[start] + numbers[end] == target {
            return vec![(start + 1) as i32, (end + 1) as i32];
        } else if numbers[start] + numbers[end] < target {
            start += 1;
        } else {
            end -= 1;
        }
    }

    vec![]
}
