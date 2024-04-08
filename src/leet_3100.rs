use core::num;

#[allow(unused_variables)]
#[allow(dead_code)]

type TestCaseType = ();
type ResultType = ();

pub struct Solution;

// Accepted
impl Solution {
    // Replace TestCaseType and ResultType with the actual type to make it easier to paste into leetcode
    pub fn max_bottles_drunk(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut sol = 0;
        let mut empties = 0;

        while (num_bottles + empties) >= num_exchange {
            while empties >= num_exchange {
                // exchange
                empties -= num_exchange;
                num_exchange += 1;
                num_bottles += 1
            }

            if num_bottles != 0 {
                // drink all
                sol += num_bottles;
                empties += num_bottles;
                num_bottles = 0;
            }
        }
        // Not enough to exchange, we have some leftover
        sol += num_bottles;

        return sol;
    }
}

pub fn test() {
    let c1: Vec<i32> = vec![13, 10];
    let c2: Vec<i32> = vec![6, 3];
    for i in 0..(c1.len() as usize) {
        let result = Solution::max_bottles_drunk(c1[i], c2[i]);
        println!("Result: {:?}", result);
    }
}
