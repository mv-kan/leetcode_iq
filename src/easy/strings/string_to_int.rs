// https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/884/
#[allow(dead_code)]
fn solve(s: String) -> i32 {
    // filter
    let unfil: Vec<char> = s.chars().collect();
    let mut chars = Vec::new();
    let mut left_spaces = true;
    let mut math_sign = ' ';
    for i in 0..unfil.len() {
        if unfil[i] == ' ' && left_spaces {
            continue
        } else {
            left_spaces = false
        }
        
        if math_sign == ' ' {
            if unfil[i] == '+' {
                math_sign = '+';
                chars.push(math_sign);
            } else if unfil[i] == '-' {
                math_sign = '-';
                chars.push(math_sign);
            } else if unfil[i] >= '0' && unfil[i] <= '9' {
                math_sign = '+';
                chars.push(math_sign);
                if unfil[i] != '0' {
                    chars.push(unfil[i]);
                }
            } else {
                break
            }
        } else {
            if unfil[i] >= '0' && unfil[i] <= '9' {
                if !((chars[chars.len()-1] == '+' || chars[chars.len()-1] =='-') &&
                unfil[i] == '0') {
                    chars.push(unfil[i])
                }
            } else {
                break
            }
        }
    }

    // parse
    let mut r: i64 = 0;
    let maxi64 = i64::MAX as f64;
    let max_power = maxi64.log10() as usize;

    for i in (0..chars.len()).rev() {
        if chars[i] == '-' {
            r *= -1
        } else if chars[i] != '+' {
            let digit = chars[i] as i64 - '0' as i64;
            let ten = 10 as i64;
            let power = chars.len() - 1 - i;
            if power > max_power {
                r = i32::MAX as i64 + 10
            } else {
                let position = ten.pow(power as u32);
                r += digit * position;
            }
        }
        if r > i32::MAX as i64 {
            if chars[0] == '+' {
                return i32::MAX
            } else {
                return i32::MIN
            }
        }
    }
    return r as i32;
}

#[allow(dead_code)]
pub fn test() {
    let s =  "1337c0d3";
    let r = solve(s.to_string());
    println!("{r}");
    
    let s =  "words and 987";
    let r = solve(s.to_string());
    println!("{r}");

    let s =  "-91283472332";
    let r = solve(s.to_string());
    println!("{r}");

    let s = "10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000522545459";
    let r = solve(s.to_string());
    println!("{r}");

    let s = "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000522545459";
    let r = solve(s.to_string());
    println!("{r}");
}