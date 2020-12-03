use std::io::BufRead;

const FILE: &str = "../day3/input.txt";

struct Slope {
    x: usize,
    y: usize,
    off_x: usize,
    trees: u32,
}

impl Slope {
    fn new(x: usize, y: usize) -> Slope {
        Slope { x, y, off_x: 0, trees: 0 }
    }
}

fn main() {
    let r = tools::must_reader(FILE);
    let slopes = &mut [
        Slope::new(1, 1),
        Slope::new(3, 1),
        Slope::new(5, 1),
        Slope::new(7, 1),
        Slope::new(1, 2)
    ];
    let mut y: usize = 0;
    for v in r.lines() {
        let line = v.unwrap();
        let len = line.len();
        for s in slopes.iter_mut() {
            if y % s.y != 0 {
                continue;
            }
            let off = s.off_x % len;
            // only works for ascii
            if &line[off..off + 1] == "#" {
                s.trees += 1
            }
            s.off_x += s.x;
        }
        y += 1;
    }
    println!("Day 3 Solution 1: {}", slopes[1].trees);
    println!("Day 3 Solution 2: {}", slopes.iter().map(|v| v.trees).product::<u32>())
}
