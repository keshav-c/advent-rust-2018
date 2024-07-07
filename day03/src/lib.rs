use std::fs;

use regex::Regex;

mod cell;
mod claim;
mod fabric;

// fn read_input(path: &str) {
//     let re = Regex::new(r"#(?P<id>\S+) @ (?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<height>\d+)")
//         .unwrap();
//     let input = fs::read_to_string(path).unwrap();
//     input.lines().map(|l| {
//         let captures = re.captures(l);
//     })
//     // #1 @ 1,3: 4x4
// }
