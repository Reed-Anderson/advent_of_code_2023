use std::fs::File;
use std::io::Read;
use std::str::Chars;

pub fn day_01_a() {
    println!("\tPart A");
    let mut file = File::open("./src/days/day_01_input.txt")
        .expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let lines = contents.lines();
    let mut sum: i32 = 0;
    for line in lines {
        sum += handle_line(line.chars()) as i32;
    }
    println!("\tSum of all calibration values is {}\n", sum);
}

pub fn handle_line(chars: Chars<'_>) -> i8 {
    let mut first_digit: i8 = -1;
    let mut last_digit: i8 = 0;
    for c in chars {
        match c.to_digit(10) {
            Some(digit) => {
                last_digit = digit as i8;
                if first_digit == -1 {
                    first_digit = digit as i8;
                }
            },
            None => {}
        }
    }
    return first_digit * 10 + last_digit;
}

