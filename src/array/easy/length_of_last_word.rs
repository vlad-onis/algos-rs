
/// Given a string s consisting of words and spaces, return the length of the last word in the string.
/// A word is a maximal substring consisting of non-space characters only.

/// https://leetcode.com/problems/length-of-last-word/description/?envType=study-plan-v2&envId=top-interview-150

pub fn length_of_last_word(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }        

    let s = s.trim();
    let words: Vec<&str> = s.split(" ").collect();

    return words[words.len() - 1].len() as i32;
}