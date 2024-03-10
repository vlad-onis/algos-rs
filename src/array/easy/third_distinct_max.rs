pub fn find_max(nums: &Vec<i32>) -> Option<i32> {
    nums.iter().max().cloned()
}

pub fn third_max(mut nums: Vec<i32>) -> i32 {
    let mut maxes_found = 0;
    let mut maxes_collection: [Option<i32>; 3] = [None; 3];

    while maxes_found <= 2 {
        let max = find_max(&nums);

        if max.is_some() && !maxes_collection.contains(&max) {
            maxes_collection[maxes_found] = max;
            maxes_found += 1;

            let position = nums.iter().position(|el| *el == max.unwrap()).unwrap();
            nums.remove(position);
        } else if max.is_some() && maxes_collection.contains(&max) {
            let position = nums.iter().position(|el| *el == max.unwrap()).unwrap();
            nums.remove(position);
        } else if max.is_none() {
            return maxes_collection[0].unwrap();
        }
    }

    maxes_collection[2].unwrap()
}

#[cfg(test)]
pub mod tests_third_max {
    use super::*;

    #[test]
    pub fn find_third_max() {
        let nums = vec![1, 2, 3, 4, 5];
        let res = third_max(nums);
        println!("{res}");
    }

    #[test]
    pub fn find_third_max_duplaicates() {
        let nums = vec![5, 4, 5, 5, 5];
        let res = third_max(nums);
        println!("{res}");
    }
}
