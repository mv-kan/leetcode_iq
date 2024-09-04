// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/578/
#[allow(dead_code)]
fn solve(nums: Vec<i32>) -> bool {
    let mut tmp = nums.clone();
    tmp.sort();
    let mut prev = tmp[0];
    for i in 1..tmp.len() {
        if prev == tmp[i] {
            return true;
        }
        prev = tmp[i];
    }
    return false;
}

#[allow(dead_code)]
pub fn test() {
    let nums = vec![1,2,3,4,5,1,6,7];
    let contains = solve(nums.clone());

    println!("{:?} - {}", nums, contains);
}