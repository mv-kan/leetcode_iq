// 
#[allow(dead_code)]
fn solve(haystack: String, needle: String) -> i32 {
    let haychars: Vec<char> = haystack.chars().collect();
    let needchars: Vec<char> = needle.chars().collect();
    for i in 0..haychars.len() {
        let mut is_substr = true;
        for j in 0..needchars.len() {
            if i + j + 1 > haychars.len() || needchars[j] != haychars[i + j] { 
                is_substr = false;
                break;
            }
        }
        if is_substr {
            return i as i32
        }
    }
    return -1
}

#[allow(dead_code)]
pub fn test() {
    let a = "ssadbutsad";
    let b = "sad";
    let r = solve(a.to_string(), b.to_string());
    println!("{r}");
}