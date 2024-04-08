#[allow(unused_variables)]
#[allow(dead_code)]

type TestCaseType = Vec<i32>;
type ResultType = i32;

pub struct Solution;

// Accepted, 100% Runtime, 100% Memory
impl Solution {
    pub fn longest_monotonic_subarray(case: Vec<i32>) -> i32 {
        let mut sol: i32 = 1;

        let mut l: usize = 0;
        let mut dl: usize = 1;

        while l + dl < case.len() {
            if case[l + dl - 1] < case[l + dl] {
                dl += 1;
            } else {
                l = l + dl;
                dl = 1
            }
            sol = sol.max(dl as i32)
        }

        l = 0;
        dl = 1;
        while l + dl < case.len() {
            if case[l + dl - 1] > case[l + dl] {
                dl += 1;
            } else {
                l = l + dl;
                dl = 1
            }
            sol = sol.max(dl as i32)
        }

        return sol;
    }
}

pub fn test() {
    let test_cases: Vec<TestCaseType> = vec![vec![1, 4, 3, 3, 2], vec![3, 3, 3, 3], vec![3, 2, 1]];
    for case in test_cases {
        let result = Solution::longest_monotonic_subarray(case);
        println!("Result: {:?}", result);
    }
}
