// https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/880/
#[allow(dead_code)]
fn solve(x: i32) -> i32 {
    let mut num = x as i64;
    let mut tmp = 0 as i64; 
    let maxi32 = i64::pow(2, 31) - 1;
    let mini32 =  i64::pow(-2, 31);
    while num != 0 {
        tmp *= 10;
        tmp += num % 10;
        num = num / 10;
        if tmp > maxi32 || tmp < mini32 {
            return 0
        }
    }
    return tmp as i32;
}

#[allow(dead_code)]
pub fn test() {
    let n2 = solve(123);
    let n1 = solve(-123);
    println!("{n1}, {n2}");
    let n3 = solve(1534236469);
    println!("{n3}");
}