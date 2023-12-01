use std::fs::File;
use std::io::Read;

use crate::days::handle_line;

pub fn day_01_b() {
    println!("\tPart B");
    let mut contents = String::new();
    let mut file = File::open("./src/days/day_01_input.txt")
        .expect("Unable to open file");
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let lines = contents.lines();
    let mut sum: i32 = 0;
    for line in lines {
        sum += handle_line(prepare_line(line).chars()) as i32;
    }
    println!("\tSum of all calibration values (when considering digits as words) is {}\n", sum);
}

fn prepare_line(line: &str) -> String {
    return line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
}
