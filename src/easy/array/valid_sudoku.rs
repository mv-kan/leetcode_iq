// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/769/
#[allow(dead_code)]
fn solve(board: Vec<Vec<char>>) -> bool {
    fn contains_dulplicates(nums: &Vec<char>) -> bool {
        let mut tmp = nums.clone();
        tmp.sort();
        let mut prev = tmp[0];
        for i in 1..tmp.len() {
            if prev == tmp[i] && prev != '.'{
                return true;
            }
            prev = tmp[i];
        }
        return false;
    }
    
    // rows 
    for i in 0..board.len() {
        if contains_dulplicates(&board[i]) {
            return false
        }
    }

    // cols 
    for i in 0..board.len() {
        let mut v = vec!['.','.','.','.','.','.','.','.','.'];
        for j in 0..board.len() {
            v[j] = board[j][i];
        }
        if contains_dulplicates(&v) {
            return false
        }
    }
    // 3 by 3 grid
    for i in 0..board.len() {
        let mut v = vec!['.','.','.','.','.','.','.','.','.'];
        for j in 0..board.len() {
            v[j] = board[i / 3 * 3 + j / 3][i % 3 * 3 + j % 3];
        }
        if contains_dulplicates(&v) {
            return false
        }
    }

    return true
}

#[allow(dead_code)]
pub fn test() {
    let sudoku = vec![
     vec!['5','3','.','.','7','.','.','.','.']
    ,vec!['6','.','.','1','9','5','.','.','.']
    ,vec!['.','9','8','.','.','.','.','6','.']
    ,vec!['8','.','.','.','6','.','.','.','3']
    ,vec!['4','.','.','8','.','3','.','.','1']
    ,vec!['7','.','.','.','2','.','.','.','6']
    ,vec!['.','6','.','.','.','.','2','8','.']
    ,vec!['.','.','.','4','1','9','.','.','5']
    ,vec!['.','.','.','.','8','.','.','7','9']];
    let r = solve(sudoku);
    println!("r={r}");
}