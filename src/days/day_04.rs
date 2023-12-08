use std::collections::HashSet;
use std::fs;

pub fn day_04() {
    if let Ok(content) = fs::read_to_string("src/days/day_04_input.txt") {
        println!("\tPart A");
        let points: i32 = content
            .lines()
            .map(|card| card_to_points(card))
            .sum();
        println!("\tThe sum of total points is {}.\n", points);

        println!("\tPart B");
        let mut extra_cards: Vec<i32> = Vec::new();
        let cards: i32 = content
            .lines()
            .map(|card| card_to_qty(card, &mut extra_cards))
            .sum();
        println!("\tThe total number of cards is {}", cards);
    } else {
        eprintln!("\tUnable to open or read file!");
    }
}

fn card_to_points(card: &str) -> i32 {
    let win_qty = get_win_qty(card);
    if win_qty == 0 {
        0
    } else {
        let base: i32 = 2;
        let points = base.pow((win_qty - 1).try_into().unwrap());
        
        points
    }
}

fn number_list_to_hash_set(opt_number_str: Option<&str>) -> HashSet<i32> {
    if let Some(number_str) = opt_number_str {
        let number_arr = number_str
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .into_iter();
        
        HashSet::from_iter(number_arr)
    } else {
        eprintln!("\tFound empty number list!");

        HashSet::new()
    }
}

fn get_win_qty(card: &str) -> usize {
    if let Some(number_section) = card.split(':').last() {
        let mut split_numbers = number_section.split('|').into_iter();
        let winning_numbers = number_list_to_hash_set(split_numbers.next());
        let my_numbers = number_list_to_hash_set(split_numbers.next());

        winning_numbers.intersection(&my_numbers).count()
    }
    else {
        eprintln!("\tUnable to read card {}", card);
        
        0
    }
}

fn card_to_qty(card: &str, extra_cards: &mut Vec<i32>) -> i32 {
    let qty_current_card = match extra_cards.len() {
        0 => 1,
        _ => 1 + extra_cards.remove(0)
    };
    let win_qty = get_win_qty(card);
    for _ in 0..qty_current_card {
        for i in 0..win_qty {
            if extra_cards.get(i).is_some() {
                extra_cards[i] += 1;
            }
            else {
                extra_cards.push(1);
            }
        }
    }

    qty_current_card
}
