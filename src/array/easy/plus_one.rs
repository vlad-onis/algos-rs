/// You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer.
/// The digits are ordered from most significant to least significant in left-to-right order.
/// The large integer does not contain any leading 0's.
/// Increment the large integer by one and return the resulting array of digits.
///

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = digits.clone();

    let mut carry = 1;

    for index in (0..digits.len()).rev() {
        let digit = digits[index] + carry;

        if digit >= 10 {
            res[index] = digit % 10;
            carry = 1;
        } else {
            res[index] = digit;
            carry = 0;
            break;
        }
    }

    if carry == 1 {
        res.insert(0, 1);
    }

    res
}

#[cfg(test)]
pub mod plus_one_tests {

    use super::*;

    #[test]
    pub fn test_3_digit_number() {
        let nums = vec![1, 2, 3];
        let res = plus_one(nums);

        assert_eq!(res, vec![1, 2, 4]);
    }

    #[test]
    pub fn test_only_zero() {
        let nums = vec![0];
        let res = plus_one(nums);

        assert_eq!(res, vec![1]);
    }

    #[test]
    pub fn test_only_nines() {
        let nums = vec![9, 9];
        let res = plus_one(nums);
        assert_eq!(res, vec![1, 0, 0]);
    }
}
