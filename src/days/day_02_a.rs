use std::fs;

pub fn day_02_a() {
    if let Ok(content) = fs::read_to_string("./src/days/day_02_input.txt") {
        println!("\tPart A");
        let sum: i32 = content
            .lines()
            .filter_map(|line| id_of_line_if_possible(line))
            .sum();
        println!("\tSum of all possible game IDs is {}\n", sum);
    } else {
        eprintln!("Unable to open or read file!");
    }
}

fn id_of_line_if_possible(line: &str) -> Option<i32> {
    let str_vec: Vec<&str> = line.split(":").collect();
    let game_id_str = str_vec[0];
    let game_results_str = str_vec[1];

    if is_game_possible(game_results_str) {
        return Some(get_line_id(game_id_str));
    } else {
        return None;
    }
}

fn is_game_possible(game_results_str: &str) -> bool {
    let invalid_game = game_results_str.split(";").find(|game| {
        let results = game.split(",");
        // let invalid_result = results.find(|result| )
        return true;
    });
    return  invalid_game.is_none();
}

fn is_singe_result_possible(single_result: &str) -> bool {
    let str_vec: Vec<&str> = single_result.split_whitespace().collect();
    let str_num = str_vec[0];
    let str_color = str_vec[1];

    return match str_color {
        "red" => str_num.parse::<i32>().unwrap() <= 12,
        "blue" => str_num.parse::<i32>().unwrap() <= 12,
        "green" => str_num.parse::<i32>().unwrap() <= 12,
        _ => false
    };
}

fn get_line_id(game_id_str: &str) -> i32 {
    let last_part = game_id_str.split_whitespace().last();
    if let Some(numeric_str) = last_part {
        return match numeric_str.parse() {
            Err(_e) => {
                eprintln!("Unable to parse game ID from {}", game_id_str);
                return 0;
            }
            Ok(parsed) => parsed,
        };
    } else {
        return 0;
    }
}
