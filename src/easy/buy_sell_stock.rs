// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/564/
#[allow(dead_code)]
fn solve(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut n = prices[0];
    for i in 1..prices.len() {
        if prices[i] > n {
            profit += prices[i] - n;
            n = prices[i];
        } else if prices[i] < n {
            n = prices[i]
        }
    }
    return profit
}

#[allow(dead_code)]
pub fn test() {
    let sol = solve(vec![1,2,3,4,5]);
    println!("sol={sol}");
}