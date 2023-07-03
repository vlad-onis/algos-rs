/// You are given 2 arrays, sorted in non-decreasing order, and 2 integers
/// m and n that represents the number of elements of each array.
/// Merge them into a single array sorted in non-decreasing order.
/// The final array should not be returned, but stored in nums1

#[allow(dead_code)]
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut res: Vec<i32> = vec![];

    if nums1.is_empty() {
        nums1.append(nums2);
        return;
    } else if nums2.is_empty() {
        return;
    }

    let mut index_nums1 = 0;
    let mut index_nums2 = 0;

    while (index_nums1 < m) && (index_nums2 < n) {
        let element1 = nums1[index_nums1 as usize];
        let element2 = nums2[index_nums2 as usize];

        if element1 < element2 {
            res.push(element1);
            index_nums1 += 1;
        } else {
            res.push(element2);
            index_nums2 += 1;
        }
    }

    while index_nums1 < m {
        res.push(nums1[index_nums1 as usize]);
        index_nums1 += 1;
    }

    while index_nums2 < n {
        res.push(nums2[index_nums2 as usize]);
        index_nums2 += 1;
    }

    nums1.clear();
    nums1.append(&mut res);
}

#[cfg(test)]
mod merge_sorted_array_tests {
    use super::*;

    #[test]
    fn test_empty_arrays() {
        let mut nums1: Vec<i32> = vec![];
        let mut nums2: Vec<i32> = vec![];
        let m = 0;
        let n = 0;

        merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![]);
    }

    #[test]
    fn test_first_array_empty() {
        let mut nums1: Vec<i32> = vec![];
        let mut nums2: Vec<i32> = vec![4, 5, 6];
        let m = 0;
        let n = 3;

        merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![4, 5, 6,]);
    }

    #[test]
    fn test_second_array_empty() {
        let mut nums1: Vec<i32> = vec![1, 2, 3];
        let mut nums2: Vec<i32> = vec![];
        let m = 3;
        let n = 0;

        merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1, 2, 3,]);
    }

    #[test]
    fn test_equal_lengths_array() {
        let mut nums1: Vec<i32> = vec![1, 2, 3];
        let mut nums2: Vec<i32> = vec![4, 5, 6];
        let m = 3;
        let n = 3;

        merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_unequal_lengths_array() {
        let mut nums1: Vec<i32> = vec![1, 2, 3];
        let mut nums2: Vec<i32> = vec![4, 5, 6, 12, 13];
        let m = 3;
        let n = 5;

        merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6, 12, 13]);
    }

    #[test]
    fn test_array_with_repeating_element() {
        let mut nums1: Vec<i32> = vec![1, 2, 3];
        let mut nums2: Vec<i32> = vec![4, 4, 4, 13];
        let m = 3;
        let n = 4;

        merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1, 2, 3, 4, 4, 4, 13]);
    }

    #[test]
    fn test_leetcode_case1() {
        let mut nums1: Vec<i32> = vec![1];
        let mut nums2: Vec<i32> = vec![];
        let m = 1;
        let n = 0;

        merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_leetcode_case2() {
        let mut nums1: Vec<i32> = vec![0];
        let mut nums2: Vec<i32> = vec![1];
        let m = 0;
        let n = 1;

        merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1]);
    }
}
