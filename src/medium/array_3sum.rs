use std::collections::HashMap;
use std::collections::HashSet;

// https://leetcode.com/explore/interview/card/top-interview-questions-medium/103/array-and-strings/776/
pub fn solve(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut pair_sum: HashMap<i32, Vec<((i32, usize), (i32, usize))>> = HashMap::new();
    let mut i = 0;
    while i < nums.len() {
        let mut j = i + 1;
        // let mut us = HashSet::new();
        while j < nums.len() {
            let mut tuple = (nums[i], nums[j]);
            if nums[i] > nums[j] {
                tuple = (nums[j], nums[i]);
            }
            pair_sum.entry(nums[i] + nums[j])
                    .or_insert_with(Vec::new)
                    .push(((tuple.0, i), (tuple.1, j)));
            // if us.insert(tuple) {
                
            // }
            j += 1;
        }
        i += 1;
    }
    println!("passed creating map");
    // let mut unique_pair_sum: HashMap<i32, Vec<((i32, usize), (i32, usize))>> = HashMap::new();
    // for (key, sums) in pair_sum.into_iter() {
    //     let mut unique_set = HashSet::new();
    //     let mut v = Vec::new();
    //     for sum in sums {
    //         if unique_set.insert(sum) {
    //             v.push(sum);
    //         }
    //     }
    //     unique_pair_sum.entry(key).or_insert(v);
    // }
    // println!("passed unique pair sum");

    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut unique_set = HashSet::new();
    for i in 0..nums.len() {
        let neg_num = -nums[i];
        match pair_sum.get(&neg_num) {
            Some(pairs) => {
                for pair in pairs {
                    let mut triple: Vec<i32> = Vec::with_capacity(3);
                    if pair.0.1 == i {
                        continue;
                    } else {
                        triple.push(pair.0 .0);
                    }
                    if pair.1.1 == i {
                        continue;
                    } else {
                        triple.push(pair.1 .0);
                    }
                    triple.push(nums[i]);
                    triple.sort();
                    if unique_set.insert(triple.clone()) {
                        result.push(triple);
                    }
                }
            }
            _ => (),
        }
    }
    println!("passed creating triplets");

    // fn unique_vectors(vectors: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    //     let mut unique_set = HashSet::new();
    //     let mut unique_vecs = Vec::new();

    //     for vec in vectors {
    //         if unique_set.insert(vec.clone()) {
    //             unique_vecs.push(vec);
    //         }
    //     }

    //     unique_vecs
    // }

    // result = unique_vectors(result);
    println!("extracting unique vectors");
    result.sort_by(|a, b| a.cmp(b));
    println!("sorting triplets");
    return result;
}
