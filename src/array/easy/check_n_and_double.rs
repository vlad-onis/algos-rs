/// Given an array arr of integers, check if there exist two indices i and j such that :
/// i != j
/// 0 <= i, j < arr.length
/// arr[i] == 2 * arr[j]

pub fn check_if_exist(arr: Vec<i32>) -> bool {
    if arr.is_empty() {
        return false;
    }

    for i in 0..arr.len() {
        for j in 0..arr.len() {
            if arr[j] == 2 * arr[i] {
                if i == j {
                    continue;
                }
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
pub mod double_exists_tests {

    use super::*;

    #[test]
    pub fn test_double_exists() {
        let a = vec![1, 2, 3, 4];
        assert!(check_if_exist(a));
    }

    #[test]
    pub fn test_double_exists_zero() {
        let a = vec![0, 1, 2, 3, 4, 0];
        assert!(check_if_exist(a));
    }

    #[test]
    pub fn test_leetcode1() {
        let a = vec![-2, 0, 10, -19, 4, 6, -8];
        assert!(!check_if_exist(a));
    }
}
