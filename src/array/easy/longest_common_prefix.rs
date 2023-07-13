use std::ffi::c_short;

/// Write a function to find the longest common prefix string amongst an array of strings.
/// If there is no common prefix, return an empty string "".
/// https://leetcode.com/problems/longest-common-prefix

fn get_shortest<'a>(strs: &'a Vec<String>) -> &'a String {
    let mut min = &strs[0];
    for word in strs {
        if word.len() < min.len() {
            min = word;
        }
    }

    min
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let shortest = get_shortest(&strs);
    let shortest = &shortest[0..shortest.len()];

    let mut lcp = shortest;

    for word in &strs {
        let mut upper_limit = lcp.len();
        while !word.starts_with(lcp) {
            if upper_limit == 0 {
                return String::from("");
            }
            upper_limit = lcp.len() - 1;

            lcp = &lcp[0..upper_limit];
        }
    }

    lcp.to_string()
}

#[cfg(test)]
pub mod lcp_tests {

    use super::*;

    #[test]
    pub fn test_empty_array() {
        let strs: Vec<String> = vec![];
        let res = longest_common_prefix(strs);
        assert_eq!(res, String::from(""));
    }

    #[test]
    pub fn test_2_words() {
        let strs = vec!["flower".to_string(), "flow".to_string()];

        let res = longest_common_prefix(strs);
        assert_eq!(res, "flow".to_ascii_lowercase());
    }

    #[test]
    pub fn test_3_words() {
        let strs = vec!["flower".to_string(), "flow".to_string(), "fl".to_string()];

        let res = longest_common_prefix(strs);
        assert_eq!(res, "fl".to_ascii_lowercase());
    }

    #[test]
    pub fn test_1_letter_common_prefix() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "ftest".to_string(),
        ];

        let res = longest_common_prefix(strs);
        assert_eq!(res, "f".to_ascii_lowercase());
    }

    #[test]
    pub fn test_no_common_prefix() {
        let strs = vec!["flower".to_string(), "flow".to_string(), "test".to_string()];

        let res = longest_common_prefix(strs);
        assert_eq!(res, "".to_ascii_lowercase());
    }

    #[test]
    pub fn test_no_common_prefix_but_common_word() {
        let strs = vec![
            "reflower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];

        let res = longest_common_prefix(strs);
        assert_eq!(res, "".to_ascii_lowercase());
    }
}
