// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/559/
#[allow(dead_code)]
fn solve(digits: Vec<i32>) -> Vec<i32> {
    let mut result = digits.clone();
    fn increment(digits: &mut Vec<i32>, pos: usize) {
        digits[pos] = (digits[pos] + 1) % 10;
        if pos == 0 && digits[pos] == 0 {
            digits.insert(0, 1);
        } else if pos == 0 {
            return;
        }
        if digits[pos] == 0 {
            increment(digits, pos - 1)
        }
    }
    let len = result.len();
    increment(&mut result, len - 1);
    return result;
}

#[allow(dead_code)]
pub fn test() {
    let num = vec![9];
    let newnum = solve(num);
    println!("{:?}", newnum);

    let num = vec![9,9,9,9,9];
    let newnum = solve(num);
    println!("{:?}", newnum);

    let num = vec![9,9,9,9,8];
    let newnum = solve(num);
    println!("{:?}", newnum);
}
