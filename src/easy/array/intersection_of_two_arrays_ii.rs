// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/674/
#[allow(dead_code)]
fn solve(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut nums1 = nums1.clone();
    let mut nums2 = nums2.clone();
    let mut inter = Vec::new();
    nums1.sort();
    nums2.sort();

    let mut i1 = 0;
    let mut i2 = 0;
    loop {
        if nums1[i1] == nums2[i2] {
            inter.push(nums1[i1]);
            i1 += 1;
            i2 += 1;
        } else if nums1[i1] > nums2[i2] {
            i2 += 1;
        } else if nums1[i1] < nums2[i2] {
            i1 += 1;
        }
        if i1 >= nums1.len() || i2 >= nums2.len() {
            break;
        }
    }

    return inter;
}

#[allow(dead_code)]
pub fn test() {
    let  nums1 = vec![1,2,2,1];
    let  nums2 = vec![2,2];
    let inter = solve(nums1, nums2);
    println!("{:?}", inter);

    let  nums1 = vec![4,9,5];
    let  nums2 = vec![9,4,9,8,4];
    let inter = solve(nums1, nums2);
    println!("{:?}", inter);
}