use std::collections::HashMap;

/// https://leetcode.com/problems/counting-elements/

pub fn count_elements(arr: Vec<i32>) -> i32 {
    if arr.iter().all(|element| *element % 2 == 0) || arr.iter().all(|element| *element % 2 != 0) {
        return 0;
    }

    let mut count_map: HashMap<i32, i32> = HashMap::new();

    for num in arr {
        *count_map.entry(num).or_insert(0) += 1;
    }

    let mut result = 0;

    for key in count_map.keys() {
        if count_map.contains_key(&(*key + 1)) {
            result += count_map[key];
        }
    }

    result
}

#[cfg(test)]
pub mod tests_count_elements {

    use super::*;

    #[test]
    pub fn test_all_even_or_odd() {
        let arr = vec![1, 3, 5, 7, 9];
        assert_eq!(count_elements(arr), 0);

        let arr = vec![0, 2, 4, 6, 8];
        assert_eq!(count_elements(arr), 0);
    }

    #[test]
    pub fn test_leetcode1() {
        let arr = vec![1, 2, 3, 5, 7, 9];
        assert_eq!(count_elements(arr), 2);
    }

    #[test]
    pub fn test_leetcode2() {
        let arr = vec![1, 1, 3, 3, 5, 5, 7, 7];
        assert_eq!(count_elements(arr), 0);
    }
}
