// https://leetcode.com/problems/reverse-string/description/

pub fn reverse_string(s: &mut Vec<char>) {
    if s.is_empty() || s.len() == 1 {
        return;
    }

    let mut start = 0;
    let mut end = s.len() - 1;

    while start < end {
        let temp_ch = s[start];
        s[start] = s[end];
        s[end] = temp_ch;
        start += 1;
        end -= 1;
    }
}
