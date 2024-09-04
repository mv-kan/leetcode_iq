
// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/546/
#[allow(dead_code)]
fn solve(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut result = Vec::new();
    let mut map = HashMap::new();
    for i in 0..nums.len() {
        map.insert(target - nums[i], i);
    }
    for i in 0..nums.len() {
        match map.get(&nums[i]) {
            Some(&idx) => {
                if idx != i {
                    if idx > i {
                        result.push(i as i32);
                        result.push(idx as i32);
                    } else {
                        result.push(idx as i32);
                        result.push(i as i32);
                    }
                    return result;
                } 
            },
            _ => {},
        }
    }
    return result
}

#[allow(dead_code)]
pub fn test() {
    let  nums = vec![2,7,11,15];
    let sum = solve(nums, 9);
    println!("{:?}", sum);
}