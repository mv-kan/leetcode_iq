// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/567/
#[allow(dead_code)]
fn solve(nums: &mut Vec<i32>) {
    // based on bubble sort 
    for i in 0..nums.len() {
        for j in 0..nums.len()-i-1 {
            if nums[j] == 0 {
                nums.swap(j, j+1);
            }
        }
    }
}

pub fn test() {
    let mut num = vec![0,1,0,3,12,0,0,0];
    solve(&mut num);
    println!("{:?}", num);
}

