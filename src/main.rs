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
fn main() {
    day_01();
}
