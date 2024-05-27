use std::collections::HashMap;

fn main() {}

fn two_sum_first_ans(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                res.push(i as i32);
                res.push(j as i32);
            }
        }
    }
    res
}

fn is_palindrome_num_first_ans(n: i32) -> bool {
    let reversed: String = n.to_string().chars().rev().collect();
    n.to_string() == reversed
}

fn roman_to_int_first_ans(s: String) -> i32 {
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
    res
}

fn score_of_string(s: String) -> i32 {
    let mut score: i32 = 0;
    let bytes: &[u8] = s.as_bytes();
    for i in 0..bytes.len() - 1 {
        score += (bytes[i] as i32 - bytes[i + 1] as i32).abs();
    }
    score
}
