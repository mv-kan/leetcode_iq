// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/564/
#[allow(dead_code)]
fn solve(s: String) -> bool {
    // filter 
    let unfiltered: Vec<char> = s.to_ascii_lowercase().chars().collect();
    let mut chars = Vec::new();
    for i in 0..unfiltered.len() {
        if (unfiltered[i] >= 65 as char && unfiltered[i] <= 90 as char) || 
            (unfiltered[i] >= 97 as char && unfiltered[i] <= 122 as char) || 
            (unfiltered[i] >= 48 as char && unfiltered[i] <= 57 as char) || 
            (unfiltered[i] >= 48 as char && unfiltered[i] <= 57 as char) {
            chars.push(unfiltered[i])
        }
    }
    
    // compare 
    let len = chars.len();
    for i in 0..(len/2) {
        let ri = chars.len()-i-1;
        if chars[i] != chars[ri] {
            return false;
        }
    }
    return true;
}

#[allow(dead_code)]
pub fn test() {
    let s = "A man, a plan, a canal: Panama";
    let r = solve(s.to_string());
    println!("{r}");
    let s = "race a car";
    let r = solve(s.to_string());
    println!("{r}");
    
    let s = "0P";
    let r = solve(s.to_string());
    println!("{r}");
}