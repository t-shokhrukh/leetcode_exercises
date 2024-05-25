use std::mem::forget;

fn main() {}

fn two_sum_first_ans(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
    // You may assume that each input would have exactly one solution, and you may not use the same element twice.
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

