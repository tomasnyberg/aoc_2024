use std::cmp::{max, min, Reverse};
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::io::{self, Read};

fn _dbg_print(disk: &[i64]) {
    for &x in disk.iter() {
        if x == -1 {
            print! {"."};
            continue;
        }
        print!("{}", x);
    }
    println!();
}

fn score(disk: &[i64], part_one: bool) -> i64 {
    let mut end = disk.iter().position(|&x| x == -1).unwrap();
    if !part_one {
        end = disk.len();
    }
    _dbg_print(&disk[..end]);
    disk.iter()
        .enumerate()
        .take(end)
        .map(|(i, &x)| max(0, x) * i as i64)
        .sum::<i64>()
}

pub fn part_one(original_disk: &Vec<i64>) -> i64 {
    let mut disk = original_disk.to_owned();
    let mut left = 0;
    let mut right = disk.len() - 1;
    while left < right {
        if disk[left] != -1 {
            left += 1;
            continue;
        }
        if disk[right] == -1 {
            right -= 1;
            continue;
        }
        disk.swap(left, right);
        left += 1;
        right -= 1;
    }
    score(&disk, true)
}

fn place_file(
    heaps: &mut [BinaryHeap<Reverse<i64>>],
    disk: &mut [i64],
    to_push: &mut Vec<(usize, i64)>,
) {
    let mut earliest = -1;
    let mut earliest_index = 11;
    for heap_size in to_push.len()..10 {
        if heaps[heap_size].is_empty() {
            continue;
        }
        let smallest = heaps[heap_size].peek().unwrap().0;
        if earliest == -1 || smallest < earliest {
            earliest = smallest;
            earliest_index = heap_size;
        }
    }
    if earliest_index == 11 {
        to_push.clear();
        return;
    }
    let diff = earliest_index - to_push.len();
    heaps[earliest_index].pop();
    to_push.iter().for_each(|&(i, _file_id)| {
        disk.swap(i, earliest as usize);
        earliest += 1;
    });
    to_push.clear();
    if diff > 0 {
        heaps[diff].push(Reverse(earliest));
    }
}

pub fn part_two(original_disk: Vec<i64>) -> i64 {
    let mut disk = original_disk.clone();
    let mut heaps: Vec<BinaryHeap<Reverse<i64>>> = (0..10).map(|_| BinaryHeap::new()).collect();
    let mut start = -1;
    let mut len = 0;
    (0..disk.len()).for_each(|i| {
        if disk[i] == -1 {
            if start == -1 {
                start = i as i64;
            }
            len += 1;
        } else {
            if start != -1 {
                heaps[len as usize].push(Reverse(start));
            }
            start = -1;
            len = 0;
        }
    });
    let mut pushed: HashSet<i64> = HashSet::new();
    let mut to_push: Vec<(usize, i64)> = vec![];
    for right in (0..disk.len()).rev() {
        let new_num = !to_push.is_empty() && to_push[0].1 != disk[right];
        if disk[right] == -1 || new_num {
            if !to_push.is_empty() {
                if pushed.contains(&to_push[0].1) {
                    to_push.clear();
                } else {
                    pushed.insert(to_push[0].1);
                }
            }
            place_file(&mut heaps, &mut disk, &mut to_push);
            if new_num && disk[right] != -1 {
                to_push.push((right, disk[right]));
            }
        } else {
            to_push.push((right, disk[right]));
        }
    }
    score(&disk, false)
}

pub fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut disk: Vec<i64> = vec![];
    let mut flag = 1;
    let mut file_id = 0;
    input.trim().chars().for_each(|c| {
        let num = c.to_digit(10).unwrap();
        let to_push = match flag {
            0 => -1,
            1 => file_id,
            _ => panic!("Invalid flag"),
        };
        for _ in 0..num {
            disk.push(to_push);
        }
        file_id += flag;
        flag = 1 - flag;
    });
    let part_one_result = part_one(&disk);
    let part_two_result = part_two(disk);
    println!("{}", part_one_result);
    println!("{}", part_two_result);
}