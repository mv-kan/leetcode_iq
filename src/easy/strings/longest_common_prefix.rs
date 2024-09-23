// https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/887/
#[allow(dead_code)]
fn solve(strs: Vec<String>) -> String {
    let mut vec_of_chars :Vec<Vec<char>>  = Vec::new();
    for i in 0..strs.len() {
        vec_of_chars.push(strs[i].chars().collect());
    }
    let mut res: Vec<char> = Vec::new();
    for i in 0..vec_of_chars[0].len() {
        let ch = vec_of_chars[0][i];
        let mut is_eq = true;
        for j in 1..strs.len() {
            if !(i < vec_of_chars[j].len() && vec_of_chars[j][i] == ch) {
                is_eq = false;
            }
        }
        if is_eq {
            res.push(ch);
        } else {
            return res.into_iter().collect();
        }
    }
    return res.into_iter().collect();
}

#[allow(dead_code)]
pub fn test() {
    // let a = "ssadbutsad";
    // let b = "sad";
    // let r = solve(vec![a.to_string(), b.to_string()]);
    // println!("{r}");

    // let a = "flower";
    // let b = "flow";
    // let c = "flight";
    // let r = solve(vec![a.to_string(), b.to_string(), c.to_string()]);
    // println!("{r}");

    // let a = "a";
    // let r = solve(vec![a.to_string()]);
    // println!("{r}");

    let a = "ab";
    let b = "a";
    let r = solve(vec![a.to_string(), b.to_string()]);
    println!("{r}");

}