// https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/881/
#[allow(dead_code)]
fn solve(s: String) -> i32 {
    let mut chars: Vec<char> = s.chars().collect();
    for i in 0..s.len() {
        if chars[i] == '0' {
            continue
        }
        let mut is_unique = true; 
        let prev_ch = chars[i];
        for j in i+1..s.len() {
            let ch = chars[j];
            if prev_ch == ch {
                chars[j] = '0';
                is_unique = false;
            }
        }
        if is_unique {
            return i as i32;
        }
    }
    return -1;
}

#[allow(dead_code)]
pub fn test() {
    let s = "loveleetcode";
    let r = solve(s.to_string());
    println!("{r}");
    let s = "aabb";
    let r = solve(s.to_string());
    println!("{r}");
}