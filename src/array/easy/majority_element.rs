use std::collections::HashMap;

/// 169. Majority Element
/// Given an array nums of size n, return the majority element.
/// The majority element is the element that appears more than ⌊n / 2⌋ times.
/// You may assume that the majority element always exists in the array.

#[allow(dead_code)]
pub fn majority_element(nums: Vec<i32>) -> i32 {
    // element, fqy
    let mut res_map: HashMap<i32, i32> = HashMap::new();
    let majority_size = nums.len() / 2;

    nums.clone().into_iter().for_each(|element| {
        *res_map.entry(element).or_insert(0) += 1;
    });

    for element in nums {
        if res_map[&element] > majority_size as i32 {
            return element;
        }
    }

    0
}

#[allow(dead_code)]
pub fn majority_element_constant_mem(nums: Vec<i32>) -> i32 {
    let (mut count, mut majority_el) = (0, 0);

    for element in nums.iter() {
        if count == 0 {
            majority_el = *element;
        }

        if majority_el == *element {
            count += 1;
        } else {
            count -= 1;
        }
    }

    majority_el
}

#[cfg(test)]
pub mod majority_element_tests {
    use super::*;

    #[test]
    fn test_majority() {
        let nums: Vec<i32> = vec![3, 2, 3];
        let res = majority_element(nums);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_majority_constant_mem() {
        let nums: Vec<i32> = vec![3, 2, 3];
        let res = majority_element_constant_mem(nums);
        assert_eq!(res, 3);
    }
}
