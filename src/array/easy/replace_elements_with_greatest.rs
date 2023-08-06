/// Given an array arr, replace every element in that array with the greatest element
/// among the elements to its right, and replace the last element with -1.
/// After doing so, return the array.
/// https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side/

fn find_max(arr: &[i32]) -> i32 {
    let mut max: i32 = arr[0];
    for el in arr {
        if *el > max {
            max = *el;
        }
    }

    max
}

pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.is_empty() {
        return arr;
    }

    for i in 0..arr.len() - 1 {
        arr[i] = find_max(&arr[i + 1..]);
    }
    let last_index = arr.len() - 1;
    arr[last_index] = -1;
    arr
}

#[cfg(test)]
pub mod replace_elements_tests {
    use super::*;

    #[test]
    fn testcase1() {
        let arr = vec![17, 18, 5, 4, 6, 1];
        let arr = replace_elements(arr);

        assert_eq!(arr, vec![18, 6, 6, 6, 1, -1]);
    }

    #[test]
    fn testcase2() {
        let arr = vec![400];
        let arr = replace_elements(arr);

        assert_eq!(arr, vec![-1]);
    }
}
