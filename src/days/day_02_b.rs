use std::cmp;
use std::fs;

pub fn day_02_b() {
    if let Ok(content) = fs::read_to_string("./src/days/day_02_input.txt") {
        println!("\tPart B");
        let sum: i32 = content
            .lines()
            .map(|game| get_min_power_of_game(game))
            .sum();
        println!("\tSum of all minimum powers is {}\n", sum);
    } else {
        eprintln!("Unable to open or read file!");
    }
}

fn get_min_power_of_game(game: &str) -> i32 {
    let mut red_min = 0;
    let mut green_min = 0;
    let mut blue_min = 0;
    let results_only_str = game.split(":").collect::<Vec<&str>>()[1];

    /* Iterate through each round */
    results_only_str.split(";").for_each(|round| {
        round.split(",").for_each(|result| {
            let split_result: Vec<&str> = result.split_whitespace().collect();
            let str_num = split_result[0];
            let str_color = split_result[1];
            match str_color {
                "red" => red_min = cmp::max(red_min, str_num.parse().unwrap()),
                "green" => green_min = cmp::max(green_min, str_num.parse().unwrap()),
                "blue" => blue_min = cmp::max(blue_min, str_num.parse().unwrap()),
                _ => {}
            }
        })
    });

    red_min * green_min * blue_min
}
