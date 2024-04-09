#[allow(unused_variables)]
#[allow(dead_code)]

type ArgType1 = Vec<i32>;
type ArgType2 = i32;
type ResultType = i32;

pub struct Solution;

// Accepted
impl Solution {
    pub fn main_function_name(nums: ArgType1, k: ArgType2) -> i32 {
        let mut l = 0;
        let mut size = 1;
        loop {
            if l + size > nums.len() {
                l = 0;
                size += 1;
                if size > nums.len() {
                    return -1;
                }
            }
            let or_sum = nums[l..l + size].iter().fold(0, |acc, x| {
                return acc | x;
            });
            if or_sum >= k {
                break;
            }
            l += 1;
        }
        return size as i32;
    }
}

pub fn test() {
    let test_cases: Vec<(ArgType1, ArgType2)> = vec![
        (vec![1, 2, 3], 2),
        (vec![2, 1, 8], 10),
        (vec![1, 12, 2, 5], 43),
    ];
    for case in test_cases {
        print!("INP : {:?}\n", case);
        let result = Solution::main_function_name(case.0, case.1);
        print!("OUT : {}\n", result);
    }
}
