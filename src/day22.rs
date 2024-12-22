use std::{
    collections::{HashMap, VecDeque},
    io::{self, Read},
};

fn evolve(mut secret: i64) -> i64 {
    secret ^= secret * 64;
    secret %= 16777216;
    secret ^= secret / 32;
    secret %= 16777216;
    secret ^= secret * 2048;
    secret %= 16777216;
    secret
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let nums: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut totals: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
    let mut part_one = 0;
    for num in nums {
        let mut curr = num;
        let mut prev_ones = curr % 10;
        let mut dq: VecDeque<i64> = VecDeque::new();
        let mut seen: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
        let mut curr_key = (-11, -11, -11, -11);
        for _ in 0..2000 {
            curr = evolve(curr);
            let ones = curr % 10;
            let diff = ones - prev_ones;
            prev_ones = ones;
            dq.push_back(diff);
            if dq.len() > 4 {
                dq.pop_front();
            }
            if dq.len() == 4 {
                curr_key = (dq[0], dq[1], dq[2], dq[3]);
            }
            if curr_key != (-11, -11, -11, -11) && !seen.contains_key(&curr_key) {
                seen.insert(curr_key, ones);
            }
        }
        part_one += curr;
        for (k, v) in seen {
            totals.insert(k, *totals.get(&k).unwrap_or(&0) + v);
        }
    }
    println!("{}", part_one);
    println!("{}", totals.values().max().unwrap());
}