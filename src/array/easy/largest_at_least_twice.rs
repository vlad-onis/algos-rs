/// https://leetcode.com/problems/largest-number-at-least-twice-of-others/

pub fn dominant_index(mut nums: Vec<i32>) -> i32 {
    let max = nums.iter().max().unwrap().to_owned();
    let max_pos = nums.iter().position(|el| *el == max).unwrap();
    nums.remove(max_pos);

    if nums.into_iter().all(|el| {
        let r = el * 2;
        println!("el = {}, max = {}, ... {:?}", el, max, max >= r);
        max >= el * 2
    }) {
        return max_pos as i32;
    }

    -1
}

#[cfg(test)]
pub mod dominant_index_tests {
    use super::dominant_index;

    #[test]
    pub fn leetcode_tc1() {
        let nums = vec![3, 6, 1, 0];
        dominant_index(nums);
    }
}
