// https://leetcode.com/explore/interview/card/top-interview-questions-easy/92/array/770/
#[allow(dead_code)]
fn solve(matrix: &mut Vec<Vec<i32>>) {
    fn rotate_outer(matrix: &mut Vec<Vec<i32>>, begin: usize, len: usize) {
        let base = begin;
        let mut tmp ;
        let mut tmp2;
        let lidx = begin + len - 1; 
        for i in 0..(lidx-begin) {
            tmp = matrix[base][base + i];
            matrix[base][base + i] = matrix[lidx - i][base];
    
            tmp2 = matrix[base + i][lidx];
            matrix[base + i][lidx] = tmp;
            tmp = tmp2;
        
            tmp2 = matrix[lidx][lidx - i];
            matrix[lidx][lidx - i] = tmp;
            tmp = tmp2;
            matrix[lidx - i][base] = tmp;
        }
    
    }
    let mut begin = 0;
    let mut len = matrix.len();
    loop {
        if len < 2 {
            break;
        }
        rotate_outer(matrix, begin, len);
        len -= 2;
        begin += 1;
        
    }
}

#[allow(dead_code)]
pub fn test() {
    let mut mat = vec![
        vec![1,2,3,4],
        vec![5,6,7,8],
        vec![9,10,11,12],
        vec![13,14,15,16],
    ];
    println!("BEFORE");
    for i in 0..mat.len() {
        println!("{:?}", mat[i]);
    }
    solve(&mut mat);
    println!("AFTER");
    for i in 0..mat.len() {
        println!("{:?}", mat[i]);
    }

    let mut mat = vec![
        vec![1,2,3],
        vec![4,5,6],
        vec![7,8,9],
    ];
    println!("BEFORE");
    for i in 0..mat.len() {
        println!("{:?}", mat[i]);
    }
    solve(&mut mat);
    println!("AFTER");
    for i in 0..mat.len() {
        println!("{:?}", mat[i]);
    }

    let mut mat = vec![
        vec![1],
    ];
    println!("BEFORE");
    for i in 0..mat.len() {
        println!("{:?}", mat[i]);
    }
    solve(&mut mat);
    println!("AFTER");
    for i in 0..mat.len() {
        println!("{:?}", mat[i]);
    }
}