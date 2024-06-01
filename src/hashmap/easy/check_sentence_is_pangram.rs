use std::collections::HashMap;

/// https://leetcode.com/problems/check-if-the-sentence-is-pangram/description/

/// A pangram is a sentence where every letter of the English alphabet appears at least once.

pub fn check_if_pangram(sentence: String) -> bool {
    let mut letter_map: HashMap<char, i32> = HashMap::new();

    // iterate over 97(a) -> 122 (z) all letters
    // and build the letter map
    (97 as char..123 as char).into_iter().for_each(|character| {
        letter_map.insert(character, 0);
    });

    for character in sentence.chars() {
        // if the letter is not in the map it means it is a differnt
        // character. We can ignore it because we only care about the
        // letters
        if !letter_map.contains_key(&character.to_ascii_lowercase()) {
            continue;
        } else {
            let val = letter_map.get_mut(&character).unwrap();
            *val += 1;
        }
    }

    println!("{:?}", letter_map);

    for key in letter_map.keys() {
        let val = letter_map[key];
        if val == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
pub mod tests_check_sentence_is_pangram {

    use super::*;

    #[test]
    pub fn test_leetcode1() {
        assert!(check_if_pangram(String::from(
            "thequickbrownfoxjumpsoverthelazydog"
        )));
    }
}
