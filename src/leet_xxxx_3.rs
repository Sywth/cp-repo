#[allow(unused_variables)]
#[allow(dead_code)]

type ArgType1 = ();
type ArgType2 = ();
type ArgType3 = ();
type ResultType = ();

pub struct Solution;

impl Solution {
    // Replace TestCaseType and ResultType with the actual type to make it easier to paste into leetcode
    pub fn main_function_name(v1: ArgType1, v2: ArgType2, v3: ArgType3) -> ResultType {
        return ();
    }
}

pub fn test() {
    let test_cases: Vec<(ArgType1, ArgType2, ArgType3)> = vec![((), (), ()), ((), (), ())];
    for case in test_cases {
        print!("INP : {:?}\n", case);
        let result = Solution::main_function_name(case.0, case.1, case.2);
        print!("OUT : {}\n", result);
    }
}
