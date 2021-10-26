use std::fs::File;
use std::io::Read;

pub fn solve() {
    let mut f = File::open("2a.txt").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    for l in content.lines().filter(|l| !l.trim().is_empty()) {
        let dash = l.find('-').unwrap();
        let space = l[dash..].find(' ').unwrap();
        let colon = l[space..].find(':').unwrap();
        let range = l[..dash].parse::<usize>().unwrap()..l[dash..space].parse::<usize>().unwrap();
    }
}
