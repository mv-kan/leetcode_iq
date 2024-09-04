//https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/549/
#[allow(dead_code)]
fn solve(nums: Vec<i32>) -> i32 {
    let mut tmp = nums.clone();
    tmp.sort();
    let mut prev = tmp[0];
    let mut unique = true;
    for i in 1..tmp.len() {
        if prev == tmp[i] {
            unique = false;
        } else if unique {
            return prev;
        } else {
            unique = true;
        }
        prev = tmp[i];
    }
    return prev;
}

#[allow(dead_code)]
pub fn test() {
    let nums = vec![4,1,2,1,2];
    let unique = solve(nums.clone());

    println!("{:?} - {}", nums, unique);
}