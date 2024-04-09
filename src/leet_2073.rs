use std::cmp::max;

#[allow(unused_variables)]
#[allow(dead_code)]

type ArgType1 = Vec<i32>;
type ArgType2 = i32;
type ResultType = i32;

pub struct Solution;

/* Unsolved fails on
tickets = [84,49,5,24,70,77,87,8]
k = 3
*/
impl Solution {
    // Replace TestCaseType and ResultType with the actual type to make it easier to paste into leetcode
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut res = 0;
        let ticket_at_k = tickets[k];
        tickets.into_iter().enumerate().for_each(|(i, t)| {
            res += if t >= ticket_at_k { ticket_at_k } else { t };
            res += if (i <= k) && (t < ticket_at_k) { -1 } else { 0 };
        });

        return res;
    }
}

pub fn test() {
    let test_cases: Vec<(ArgType1, ArgType2)> = vec![(vec![2, 3, 2], 2), (vec![5, 1, 1, 1], 0)];
    for case in test_cases {
        print!("INP : {:?}\n", case);
        let result = Solution::time_required_to_buy(case.0, case.1);
        print!("OUT : {}\n", result);
    }
}
