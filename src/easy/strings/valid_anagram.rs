// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/564/
#[allow(dead_code)]
fn solve(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut chars1: Vec<char> = s.chars().collect();
    let mut chars2: Vec<char> = t.chars().collect();
    chars1.sort_unstable();
    chars2.sort_unstable();
    for i in 0..s.len() {
        if chars1[i] != chars2[i] { 
            return false;
        }
    }
    return true
}

#[allow(dead_code)]
pub fn test() {
    let s = "rat";
    let t = "tar";
    let r = solve(s.to_string(), t.to_string());
    println!("{r}");
    let s = "anagram";
    let t = "nagaram";
    let r = solve(s.to_string(), t.to_string());
    println!("{r}");
    let s = "cat";
    let t = "rat";
    let r = solve(s.to_string(), t.to_string());
    println!("{r}");
}