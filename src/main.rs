use std::collections::HashMap;

fn main() {}

fn two_sum_first_ans(nums: Vec<i32>, target: i32) -> Vec<i32> {
    /* Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
     You may assume that each input would have exactly one solution, and you may not use the same element twice.*/
    let mut res: Vec<i32> = Vec::new();
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                res.push(i as i32);
                res.push(j as i32);
            }
        }
    }
    return res;
}

fn is_palindrome_num_first_ans(x: i32) -> bool {
    // Given an integer x, return true if x is a palindrome, and false otherwise.
    let reversed: String = x.to_string().chars().rev().collect();
    x.to_string() == reversed
}

fn roman_to_int_first_ans(s: String) -> i32 {
    /*  Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
        Given a roman numeral, convert it to an integer.
         I             1
         V             5
         X             10
         L             50
         C             100
         D             500
         M             1000
     */
    let mut symbols: HashMap<char, i32> = HashMap::new();
    symbols.insert('I', 1);
    symbols.insert('V', 5);
    symbols.insert('X', 10);
    symbols.insert('L', 50);
    symbols.insert('C', 100);
    symbols.insert('D', 500);
    symbols.insert('M', 1000);

    let mut res: i32 = 0;
    let mut prev_value: i32 = 0;

    for c in s.chars().rev() {
        if let Some(&value) = symbols.get(&c) {
            if value < prev_value {
                res -= value;
            } else {
                res += value;
            }
            prev_value = value;
        }
    }
    return res;
}
