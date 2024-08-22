// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/727/
pub fn solve(nums: &mut Vec<i32>) -> i32 {
    let mut result = 0;
    let mut i = 1;
    let mut prev = nums[0];
    let mut index = 0; 
    while i < nums.len() {
        if prev == nums[i] {
            result +=1;
        } else {
            nums[index] = prev;
            index += 1;
            prev = nums[i];
        }
        i+=1;
    }
    nums[index] = prev;
    return nums.len() as i32 - result;
}