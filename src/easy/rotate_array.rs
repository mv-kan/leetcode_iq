// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/646/
#[allow(dead_code)]
fn solve(nums: &mut Vec<i32>, k: i32) {
    let mut next_hop = nums[0];
    let mut odd_case = 0;
    for i in 1..nums.len()+1 {
        let offset = (k as usize*i)%nums.len() + odd_case;
        
        let tmp = nums[offset];
        nums[offset] = next_hop;
        next_hop = tmp; 
        if offset == 0 && k % 2 == 0 && nums.len() > 1 {
            next_hop = nums[1];
            odd_case = 1;
        }
    }
}

#[allow(dead_code)]
pub fn test() {
    // let mut nums = vec![1,2,3,4,5,6,7];
    // solve(&mut nums, 3);
    // print!("{:?}", nums);
    // let mut nums = vec![-1, -100, 3, 99];
    // solve(&mut nums, 2);
    // println!("{:?}", nums);
    
    let mut nums = vec![1,2,3,4,5,6];
    solve(&mut nums, 3);
    println!("{:?}", nums);
    // let mut nums = vec![1];
    // solve(&mut nums, 0);
    // println!("{:?}", nums);
}