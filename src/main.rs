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
fn main() {
    day_01();
    day_02();
}
