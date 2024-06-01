pub fn str_str(haystack: String, needle: String) -> i32 {

    if haystack.is_empty() || needle.is_empty() {
        return -1;
    }

    if haystack == needle {
        return 0;
    }

    if needle.len() > haystack.len() {
        return -1;
    }

    let window_size = needle.len();

    let mut start = 0;

    while start <= haystack.len() - window_size {
        let slice = &haystack[start..start+window_size];
        if slice == &needle {
            return start as i32;
        }

        start += 1;
    }

    -1

}

#[cfg(test)]
pub mod str_str_tests {
    use super::*;
    
    #[test]
    pub fn leetcode_tc1() {
        let haystack =  String::from("abc");
        let needle = String::from("c");

        println!("{}",str_str(haystack, needle));
    }
}