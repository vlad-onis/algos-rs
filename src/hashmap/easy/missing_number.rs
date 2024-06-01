use std::collections::HashMap;

/// https://leetcode.com/problems/missing-number/description/

pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut res: HashMap<i32, i32> = HashMap::new();

    for num in nums.iter() {
        res.entry(*num).or_insert(0);
    }

    for i in 0..nums.len() + 1 {
        if !res.contains_key(&(i as i32)) {
            return i as i32;
        }
    }

    0
}
