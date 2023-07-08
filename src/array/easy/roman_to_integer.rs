use std::collections::HashMap;

/// Given a roman numeral, convert it to an integer.
/// Rules:
/// I can be placed before V (5) and X (10) to make 4 and 9.
/// X can be placed before L (50) and C (100) to make 40 and 90.
/// C can be placed before D (500) and M (1000) to make 400 and 900.
/// https://leetcode.com/problems/roman-to-integer/

#[allow(dead_code)]
pub fn roman_to_int(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let mut digits: HashMap<char, i32> = HashMap::new();
    digits.insert('I', 1);
    digits.insert('V', 5);
    digits.insert('X', 10);
    digits.insert('L', 50);
    digits.insert('C', 100);
    digits.insert('D', 500);
    digits.insert('M', 1000);

    let characters: Vec<char> = s.chars().collect();
    let mut index = 0;
    let digit_size = characters.len();

    let mut final_number = 0;

    while index < digit_size {
        let digit = characters[index];
        let next_digit = characters.get(index + 1);

        final_number += match digit {
            'I' => {
                if next_digit.is_some() {
                    let next_digit = next_digit.unwrap();
                    if next_digit == &'V' {
                        index += 1;
                        4
                    } else if next_digit == &'X' {
                        index += 1;
                        9
                    } else {
                        1
                    }
                } else {
                    1
                }
            }
            'X' => {
                if next_digit.is_some() {
                    let next_digit = next_digit.unwrap();
                    if next_digit == &'L' {
                        index += 1;
                        40
                    } else if next_digit == &'C' {
                        index += 1;
                        90
                    } else {
                        10
                    }
                } else {
                    10
                }
            }
            'C' => {
                if next_digit.is_some() {
                    let next_digit = next_digit.unwrap();
                    if next_digit == &'D' {
                        index += 1;
                        400
                    } else if next_digit == &'M' {
                        index += 1;
                        900
                    } else {
                        100
                    }
                } else {
                    100
                }
            }
            c => *digits.get(&c).unwrap_or(&0),
        };

        index += 1;
    }

    final_number
}

#[cfg(test)]
pub mod roman_to_integer_tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let number = String::from("");
        let res = roman_to_int(number);

        assert_eq!(res, 0);
    }

    #[test]
    fn test_digits() {
        let one = String::from("I");
        let five = String::from("V");
        let ten = String::from("X");
        let fithy = String::from("L");
        let one_hundred = String::from("C");
        let five_hundrer = String::from("D");
        let one_thousand = String::from("M");

        let res = roman_to_int(one);
        assert_eq!(res, 1);
        let res = roman_to_int(five);
        assert_eq!(res, 5);
        let res = roman_to_int(ten);
        assert_eq!(res, 10);
        let res = roman_to_int(fithy);
        assert_eq!(res, 50);
        let res = roman_to_int(one_hundred);
        assert_eq!(res, 100);
        let res = roman_to_int(five_hundrer);
        assert_eq!(res, 500);
        let res = roman_to_int(one_thousand);
        assert_eq!(res, 1000);
    }

    #[test]
    fn test_4() {
        let number = String::from("IV");
        let res = roman_to_int(number);

        assert_eq!(res, 4);
    }

    #[test]
    fn test_9() {
        let number = String::from("IX");
        let res = roman_to_int(number);

        assert_eq!(res, 9);
    }

    #[test]
    fn test_0() {
        let number = String::from("");
        let res = roman_to_int(number);

        assert_eq!(res, 0);
    }

    #[test]
    fn test_40() {
        let number = String::from("XL");
        let res = roman_to_int(number);

        assert_eq!(res, 40);
    }

    #[test]
    fn test_90() {
        let number = String::from("XC");
        let res = roman_to_int(number);

        assert_eq!(res, 90);
    }

    #[test]
    fn test_400() {
        let number = String::from("CD");
        let res = roman_to_int(number);

        assert_eq!(res, 400);
    }

    #[test]
    fn test_900() {
        let number = String::from("CM");
        let res = roman_to_int(number);

        assert_eq!(res, 900);
    }

    #[test]
    fn test_11() {
        let eleven = String::from("XI");
        let res = roman_to_int(eleven);

        assert_eq!(res, 11);
    }
}
