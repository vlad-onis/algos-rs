/// https://leetcode.com/problems/add-binary/description/

pub fn convert(c: char) -> u8 {
    if c == '0' {
        0
    } else {
        1
    }
}

pub fn add_binary(a: String, b: String) -> String {
    let res = u128::from_str_radix(&a, 2).unwrap() + u128::from_str_radix(&b, 2).unwrap();
    format!("{:b}", res)
}

#[cfg(test)]
pub mod add_binary_tests {
    use super::add_binary;

    #[test]
    pub fn test_binary_add() {
        let a = format!("100");
        let b = format!("101");

        let res = add_binary(a, b);
        assert_eq!(res, String::from("1001"));
    }
}
