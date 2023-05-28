use std::fs;

fn main() {
    println!("\nAdvent of Code - Day 3");

    let rucksacks = fs::read_to_string("rucksacks.txt").expect("Expected to read file");

    let mut sum: u32 = 0;

    for rucksack in rucksacks.lines() {
        println!("\nRucksack: \"{rucksack}\"");
        let pocket_length = rucksack.to_string().len() / 2;

        let pocket1 = &rucksack[0..pocket_length];
        let pocket2 = &rucksack[pocket_length..pocket_length * 2];

        // find the duplicate
        'find_loop: for p1_item in pocket1.chars() {
            for p2_item in pocket2.chars() {
                if p1_item == p2_item {
                    let p_dec = p1_item as u32;
                    println!("Duplicate is {p1_item} ({p_dec})");
                    
                    let priority;

                    if p_dec > 90 { // lowercase
                        priority = p_dec - 96;
                    } else { // uppercase
                        priority = p_dec - 38;
                    }

                    sum += priority;
                    break 'find_loop;
                }
            }
        }
    }

    println!("\nTotal priority: {sum}");
}
