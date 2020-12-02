use std::collections::HashSet;
use tools::{TypeRead, TypeReader};

const TARGET: i32 = 2020;
const FILE: &str = "../day1/input.txt";

fn one(p: &str) {
    let r = TypeReader::new(tools::must_reader(p));
    let mut nums: HashSet<i32> = HashSet::new();
    for value in r.lines_t::<i32>() {
        // idc let it panic
        let i_val: i32 = value.unwrap();
        let pv: i32 = TARGET - i_val;
        if nums.contains(&pv) {
            println!("Part 1: {}, Values: {}, {}", pv * i_val, pv, i_val);
            return;
        }
        nums.insert(i_val);
    }
    println!("Part 1: No Answer");
}

fn two(p: &str) {
    let r = TypeReader::new(tools::must_reader(p));
    let mut nums: Vec<i32> = Vec::new();
    for value in r.lines_t::<i32>() {
        nums.push(value.unwrap());
    }
    nums.sort();
    for i in 0..nums.len() - 2 {
        let mut last = nums.len() - 1;
        let mut next = i + 1;
        while next < last {
            let sum = nums[i] + nums[next] + nums[last];
            if sum == TARGET {
                println!("Part 2:{}, Values: {}, {}, {}", (nums[i] * nums[next] * nums[last]), nums[i], nums[next], nums[last]);
                return;
            } else if sum > TARGET {
                last -= 1
            } else {
                next += 1
            }
        }
    }
    println!("Part 2: No answer");
}

fn main() {
    one(FILE);
    two(FILE);
}
