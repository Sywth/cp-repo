#[allow(unused_variables)]
#[allow(dead_code)]
type TestCaseType = i32;
type ResultType = i32;

pub struct Solution;

// Accepted,12.24 % Runtime ,13.27 % Memory
impl Solution {
    // Replace TestCaseType and ResultType with the actual type to make it easier to paste into leetcode
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let mut sum: i32 = 0;
        let mut num = x;
        while num != 0 {
            sum += num % 10;
            num = num / 10;
        }

        return if x % sum == 0 { sum } else { -1 };
    }
}

pub fn test() {
    let test_cases: Vec<TestCaseType> = vec![18, 23];
    for case in test_cases {
        let result = Solution::sum_of_the_digits_of_harshad_number(case);
        println!("Result: {:?}", result);
    }
}
