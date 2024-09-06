use std::{
    fs::File,
    io::{prelude::*, BufReader},
    ops::Div,
};

fn day_01() {
    let file = File::open("input/input_01.txt").unwrap();
    let reader = BufReader::new(file);
    let nums: Vec<i64> = reader
        .lines()
        .map_while(Result::ok)
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    println!(
        "{} {}",
        nums.iter().map(|x| x.div(3) - 2).sum::<i64>(),
        nums.iter()
            .map(|x| {
                let mut n = (*x).div(3) - 2;
                let mut sum = 0;
                while n > 0 {
                    sum += n;
                    n = n.div(3) - 2;
                }
                sum
            })
            .sum::<i64>()
    )
}

struct Intcode {
    m: Vec<i64>,
    ip: usize,
}

impl Intcode {
    fn new(program: &Vec<i64>) -> Intcode {
        Intcode {
            m: program.to_owned(),
            ip: 0,
        }
    }
    fn step(&mut self) -> bool {
        let op = self.m[self.ip];
        match op {
            1 => {
                let addr1 = self.m[self.ip + 1];
                let addr2 = self.m[self.ip + 2];
                let addr3 = self.m[self.ip + 3];
                let val1 = self.m[addr1 as usize];
                let val2 = self.m[addr2 as usize];
                self.m[addr3 as usize] = val1 + val2;
                self.ip += 4;
                false
            }
            2 => {
                let addr1 = self.m[self.ip + 1];
                let addr2 = self.m[self.ip + 2];
                let addr3 = self.m[self.ip + 3];
                let val1 = self.m[addr1 as usize];
                let val2 = self.m[addr2 as usize];
                self.m[addr3 as usize] = val1 * val2;
                self.ip += 4;
                false
            }
            99 => true,
            _ => panic!("Unrecognized opcode"),
        }
    }
    fn run(&mut self) {
        loop {
            let halt = self.step();
            if halt {
                break;
            }
        }
    }
}

fn day_02() {
    let file = File::open("input/input_02.txt").unwrap();
    let reader = BufReader::new(file);
    let program: Vec<i64> = reader
        .lines()
        .map_while(Result::ok)
        .next()
        .map(|x| x.split(",").map(|y| y.parse::<i64>().unwrap()).collect())
        .unwrap();
    let mut machine: Intcode = Intcode::new(&program);
    machine.m[1] = 12;
    machine.m[2] = 2;
    machine.run();
    print!("{:?} ", machine.m[0]);
    'outer: for x in 0..100 {
        for y in 0..100 {
            let mut machine: Intcode = Intcode::new(&program);
            machine.m[1] = x;
            machine.m[2] = y;
            machine.run();
            if machine.m[0] == 19690720 {
                println!("{:?}", 100 * x + y);
                break 'outer;
            }
        }
    }
}

fn day_03() {
    let file = File::open("input/input_03.txt").unwrap();
    let reader = BufReader::new(file);

    #[derive(Debug, Clone, Copy)]
    enum Dir {
        U,
        D,
        L,
        R,
    }
    #[derive(Debug, Clone, Copy)]
    struct Segment {
        start: (i64, i64),
        dir: Dir,
        len: i64,
    }

    impl Segment {
        fn end(self) -> (i64, i64) {
            match self.dir {
                Dir::U => (self.start.0, self.start.1 + self.len),
                Dir::D => (self.start.0, self.start.1 - self.len),
                Dir::L => (self.start.0 - self.len, self.start.1),
                Dir::R => (self.start.0 + self.len, self.start.1),
            }
        }
    }

    fn check(x1: i64, y1: i64, _x2: i64, y2: i64, x3: i64, y3: i64, x4: i64) -> bool {
        y1 <= y3 && y2 >= y3 && x3 <= x1 && x4 >= x1
            || y2 <= y3 && y1 >= y3 && x3 <= x1 && x4 >= x1
            || y1 <= y3 && y2 >= y3 && x4 <= x1 && x3 >= x1
            || y2 <= y3 && y1 >= y3 && x4 <= x1 && x3 >= x1
    }

    fn intersect(s1: &Segment, s2: &Segment) -> Option<(i64, i64)> {
        match (s1.dir, s2.dir) {
            (Dir::U, Dir::L) | (Dir::U, Dir::R) | (Dir::D, Dir::L) | (Dir::D, Dir::R) => {
                let e1 = s1.end();
                let e2 = s2.end();
                if check(
                    s1.start.0, s1.start.1, e1.0, e1.1, s2.start.0, s2.start.1, e2.0,
                ) {
                    Some((s1.start.0, s2.start.1))
                } else {
                    None
                }
            }
            (Dir::L, Dir::U) | (Dir::R, Dir::U) | (Dir::L, Dir::D) | (Dir::R, Dir::D) => {
                let e1 = s1.end();
                let e2 = s2.end();
                if check(
                    s2.start.0, s2.start.1, e2.0, e2.1, s1.start.0, s1.start.1, e1.0,
                ) {
                    Some((s2.start.0, s1.start.1))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    let circuits: Vec<Vec<Segment>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|l| {
            let mut start = (0, 0);
            l.trim()
                .split(",")
                .map(|x| {
                    let d = match x.chars().next().unwrap() {
                        'U' => Dir::U,
                        'D' => Dir::D,
                        'L' => Dir::L,
                        'R' => Dir::R,
                        _ => panic!("Wrong direction!"),
                    };
                    let l = x
                        .chars()
                        .skip(1)
                        .collect::<String>()
                        .parse::<i64>()
                        .unwrap();
                    let s = Segment {
                        start,
                        dir: d,
                        len: l,
                    };
                    start = match d {
                        Dir::U => (start.0, start.1 + l),
                        Dir::D => (start.0, start.1 - l),
                        Dir::L => (start.0 - l, start.1),
                        Dir::R => (start.0 + l, start.1),
                    };
                    s
                })
                .collect()
        })
        .collect();
    let intersection: i64 = circuits[0]
        .iter()
        .flat_map(|s1| {
            circuits[1]
                .iter()
                .filter_map(|s2| intersect(s1, s2))
                .collect::<Vec<(i64, i64)>>()
        })
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap();
    println!("{:?}", intersection);
}

fn main() {
    day_01();
    day_02();
    day_03();
}
