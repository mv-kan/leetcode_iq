use std::collections::HashMap;
use std::collections::HashSet;
 
// https://leetcode.com/explore/interview/card/top-interview-questions-medium/103/array-and-strings/776/
pub fn solve(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut pair_sum: HashMap<i32, Vec<Vec<(i32, usize)>>> = HashMap::new();
    let mut i = 0;
    fn add_vec(map: &mut HashMap<i32, Vec<Vec<(i32, usize)>>>, key: i32, vec: Vec<(i32, usize)>) {
        map.entry(key).or_insert_with(Vec::new).push(vec);
    }
    while i < nums.len() {
        let mut j = i + 1;
        while j < nums.len() {
            add_vec(&mut pair_sum, nums[i] + nums[j], vec![(nums[i], i), (nums[j], j)]);
            j += 1;
        }
        i += 1;
    }
    println!("passed creating map ");
    let mut result: Vec<Vec<i32>> = Vec::new();

    for i in 0..nums.len() {
        let neg_num = -nums[i];
        match pair_sum.get(&neg_num) {
            Some(pairs) => {
                'out: for pair in pairs {
                    let mut triple: Vec<i32> = Vec::new();
                    for j in 0..pair.len() {
                        if pair[j].1 == i {
                            continue 'out;
                        } else {
                            triple.push(pair[j].0);
                        }
                    }
                    triple.push(nums[i]);
                    triple.sort();
                    result.push(triple);
                }
            }
            _ => (),
        }
    }
    println!("passed creating triplets");
 
    fn unique_vectors(vectors: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut unique_set = HashSet::new();
        let mut unique_vecs = Vec::new();
    
        for vec in vectors {
            if unique_set.insert(vec.clone()) {
                unique_vecs.push(vec);
            }
        }
    
        unique_vecs
    }

    result = unique_vectors(result);
    println!("extracting unique vectors");
    result.sort_by(|a, b| a.cmp(b));
    println!("sorting triplets");
    return result;
}
