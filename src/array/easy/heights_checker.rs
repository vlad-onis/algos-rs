/// https://leetcode.com/problems/height-checker/

pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut sorted = heights.clone();
    sorted.sort();

    let mut cnt = 0;
    for index in 0..heights.len() {
        if heights[index] != sorted[index] {
            cnt += 1;
        }
    }

    cnt
}
