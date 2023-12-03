use std::fs;

pub fn day_02_a() {
    if let Ok(content) = fs::read_to_string("./src/days/day_02_input.txt") {
        println!("\tPart A");
        let sum: i32 = content
            .lines()
            .filter_map(|game| id_of_game_if_possible(game))
            .sum();
        println!("\tSum of all possible game IDs is {}\n", sum);
    } else {
        eprintln!("Unable to open or read file!");
    }
}

fn id_of_game_if_possible(game: &str) -> Option<i32> {
    let split_game: Vec<&str> = game.split(":").collect();
    let game_id_str = split_game.get(0)?;
    let game_results_str = split_game.get(1)?;

    if is_game_possible(game_results_str) {
        return get_line_id(game_id_str);
    } else {
        return None;
    }
}

fn is_game_possible(game_results_str: &str) -> bool {
    game_results_str
        .split(";")
        .find(|round| !is_round_possible(round))
        .is_none()
}

fn is_round_possible(round_str: &str) -> bool {
    round_str
        .split(",")
        .find(|result| !is_result_possible(result))
        .is_none()
}

fn is_result_possible(result_str: &str) -> bool {
    let str_vec: Vec<&str> = result_str.split_whitespace().collect();
    let str_num = str_vec[0];
    let str_color = str_vec[1];

    return match str_color {
        "red" => str_num.parse::<i32>().unwrap() <= 12,
        "green" => str_num.parse::<i32>().unwrap() <= 13,
        "blue" => str_num.parse::<i32>().unwrap() <= 14,
        _ => false,
    };
}

fn get_line_id(game_id_str: &str) -> Option<i32> {
    let last_part = game_id_str.split_whitespace().last()?;
    let parsed = last_part.parse().ok()?;

    Some(parsed)
}
