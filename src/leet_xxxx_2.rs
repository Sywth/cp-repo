#[allow(unused_variables)]
#[allow(dead_code)]

type ArgType1 = ();
type ArgType2 = ();
type ResultType = ();

pub struct Solution;

impl Solution {
    // Replace TestCaseType and ResultType with the actual type to make it easier to paste into leetcode
    pub fn main_function_name(v1: ArgType1, v2: ArgType2) -> ResultType {
        return ();
    }
}

pub fn test() {
    let test_cases: Vec<(ArgType1, ArgType2)> = vec![((), ()), ((), ())];
    for case in test_cases {
        print!("INP : {:?}\n", case);
        let result = Solution::main_function_name(case.0, case.1);
        print!("OUT : {}\n", result);
    }
}
