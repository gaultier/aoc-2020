use std::fs::File;
use std::io::Read;

use itertools::Itertools;

pub fn solve() {
    let mut f = File::open("1a.txt").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    let numbers = content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    println!(
        "{}",
        numbers
            .iter()
            .cartesian_product(&numbers)
            .find(|(a, b)| *a + *b == 2020)
            .map(|(a, b)| a * b)
            .unwrap()
    );
}
