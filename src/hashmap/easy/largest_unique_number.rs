use std::collections::HashMap;

/// https://leetcode.com/problems/largest-unique-number/

pub fn largest_unique_number(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return -1;
    }

    let mut number_map: HashMap<i32, i32> = HashMap::new();

    for num in nums {
        *number_map.entry(num).or_insert(0) += 1;
    }

    let mut maximum = -1;

    for (key, value) in number_map {
        if value == 1 {
            maximum = key.max(maximum);
        }
    }

    maximum
}

#[cfg(test)]
pub mod tests_largest_unique_number {
    use super::*;

    pub fn test_leetcode1() {}
}
