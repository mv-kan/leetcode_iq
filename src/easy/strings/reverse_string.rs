// https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/
#[allow(dead_code)]
fn solve(s: &mut Vec<char>) {
    let len = s.len();
    for i in 0..(len/2) {
        let tmp = s[i];
        s[i] = s[s.len()-i-1];
        s[len-i-1] = tmp;
    }
}

#[allow(dead_code)]
pub fn test() {
    let mut v = vec!['h','e','l','l','o'];
    solve(&mut v);
    println!("{:?}", v);
}