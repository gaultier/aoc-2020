use std::fs::File;
use std::io::Read;

pub fn solve() {
    let mut f = File::open("2a.txt").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    for l in content.lines().filter(|l| !l.trim().is_empty()) {
        let dash = l.find('-').unwrap();
        let start_range = &l[..dash];
        let space = l.find(' ').unwrap();
        let end_range = &l[dash + 1..space];

        let range = start_range.parse::<usize>().unwrap()..end_range.parse::<usize>().unwrap();
        let colon = l.find(':').unwrap();
    }
}
