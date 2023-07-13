/// Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
/// https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/

pub fn str_str(haystack: String, needle: String) -> i32 {
    if haystack.is_empty() || needle.is_empty() {
        return -1;
    }

    haystack
        .find(&needle)
        .map(|index| index as i32)
        .unwrap_or(-1)
}

#[cfg(test)]
pub mod first_occurence_tests {

    use super::*;

    #[test]
    pub fn test_empty_strings() {
        let needle = String::new();
        let haystack = String::new();

        let res = str_str(haystack, needle);
        assert_eq!(res, -1);
    }

    #[test]
    pub fn test_needle_not_in_haystack() {
        let needle = String::from("test");
        let haystack = String::from("Not containing needle");

        let res = str_str(haystack, needle);
        assert_eq!(res, -1);
    }

    #[test]
    pub fn test_needle_in_haystack_only_once() {
        let needle = String::from("test");
        let haystack = String::from("containing test");

        let res = str_str(haystack, needle);
        assert_eq!(res, 11);
    }

    #[test]
    pub fn test_needle_in_haystack_twice() {
        let needle = String::from("test");
        let haystack = String::from("containing test and also contains test");

        let res = str_str(haystack, needle);
        assert_eq!(res, 11);
    }
}
