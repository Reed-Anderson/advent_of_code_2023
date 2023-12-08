use std::collections::{HashMap, HashSet};
use std::fs;

pub fn day_03() {
    if let Ok(content) = fs::read_to_string("src/days/day_03_input.txt") {
        let (symbol_lookup, part_numbers, gear_coords) = create_lookups_from_input(&content);

        println!("\tPart A");
        let sum_of_valid_pns: i32 = part_numbers
            .iter()
            .filter_map(|part_number| part_number_if_valid(part_number, &symbol_lookup))
            .sum();
        println!("\tThe sum of valid part numbers is {}\n", sum_of_valid_pns);

        println!("\tPart B");
        let sum_of_gear_ratios: i32 = gear_coords
            .iter()
            .filter_map(|gear_coord| gear_ratio_if_valid(gear_coord, &part_numbers))
            .sum();
        println!("\tThe sum of gear ratios is {}\n", sum_of_gear_ratios)
    } else {
        eprintln!("Unable to open or read file!");
    }
}

/**
 * To avoid iterating multiple times, create symbol_lookup and part_numbers.
 *
 * 	symbol_lookup represents the indices of valid symbols
 *  part_numbers is a Vector of (i32, Vector) tuples
 * 		First element represents the value of the part number
 * 		Second element represents adjecent indices
 *  gear_coords is a Vector of (usize, usize) represent coordinates where gears
 *  	are found
 */
pub fn create_lookups_from_input(
    input: &str,
) -> (
    HashMap<usize, HashSet<usize>>,
    Vec<(i32, Vec<(usize, usize)>)>,
    Vec<(usize, usize)>,
) {
    let mut symbol_lookup: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut part_numbers: Vec<(i32, Vec<(usize, usize)>)> = Vec::new();
    let mut gear_coords: Vec<(usize, usize)> = Vec::new();
    input.lines().enumerate().for_each(|(line_idx, line)| {
        let mut num_str = "".to_string();
        let mut num_start_idx: Option<usize> = None;

        line.chars().enumerate().for_each(|(char_idx, char)| {
            if char.is_digit(10) {
                if num_str.len() == 0 {
                    num_start_idx = Some(char_idx);
                }
                num_str += &char.to_string();
            } else {
                handle_part_num_end(
                    &mut num_str,
                    &mut num_start_idx,
                    line_idx,
                    &mut part_numbers,
                );
                if char != '.' {
                    let char_set = symbol_lookup
                        .entry(line_idx)
                        .or_insert_with(|| HashSet::new());
                    char_set.insert(char_idx);
                }
                if char == '*' {
                    gear_coords.push((line_idx, char_idx));
                }
            }
        });
        handle_part_num_end(
            &mut num_str,
            &mut num_start_idx,
            line_idx,
            &mut part_numbers,
        );
    });

    (symbol_lookup, part_numbers, gear_coords)
}

fn handle_part_num_end(
    num_str: &mut String,
    num_start_idx: &mut Option<usize>,
    line_idx: usize,
    part_numbers: &mut Vec<(i32, Vec<(usize, usize)>)>,
) {
    if num_start_idx.is_some() {
        let mut coordinates: Vec<(usize, usize)> = Vec::new();
        let mut i = num_start_idx.unwrap();
        if i == 0 {
            i += 1;
        }
        for char_index in i - 1..=num_start_idx.unwrap() + num_str.len() {
            if line_idx > 0 {
                coordinates.push((line_idx - 1, char_index));
            }
            coordinates.push((line_idx + 1, char_index));
        }
        if num_start_idx.unwrap() > 0 {
            coordinates.push((line_idx, num_start_idx.unwrap() - 1));
        }
        coordinates.push((line_idx, num_start_idx.unwrap() + num_str.len()));
        part_numbers.push((num_str.parse::<i32>().unwrap(), coordinates));
    }

    num_str.clear();
    *num_start_idx = None;
}

fn part_number_if_valid(
    part_number: &(i32, Vec<(usize, usize)>),
    symbol_lookup: &HashMap<usize, HashSet<usize>>,
) -> Option<i32> {
    let (part_number, coords) = part_number;
    let some_coord_valid = coords
        .iter()
        .find(|(x, y)| symbol_lookup.get(x).is_some_and(|set| set.contains(y)));

    some_coord_valid.map(|_| *part_number)
}

fn gear_ratio_if_valid(
    gear_coord: &(usize, usize),
    part_numbers: &Vec<(i32, Vec<(usize, usize)>)>,
) -> Option<i32> {
    let adjacent_pns: Vec<&(i32, Vec<(usize, usize)>)> = part_numbers
        .iter()
        .filter(|(_, pn_coords)| {
            pn_coords
                .iter()
                .find(|pn_coord| pn_coord.0 == gear_coord.0 && pn_coord.1 == gear_coord.1)
                .is_some()
        })
        .collect();
    if adjacent_pns.len() == 2 {
        let nums: Vec<i32> = adjacent_pns.iter().map(|(pn, _)| *pn).collect();
        return Some(nums.iter().fold(1, |acc, pn| acc * pn));
    }
    None
}
