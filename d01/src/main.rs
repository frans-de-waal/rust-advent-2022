use std::fs;

fn main() {
    println!("Advent of Code - Day 1");

    const FILE_PATH: &str = "calories.txt";

    // read the file
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    println!("\nCalory list:\n{contents}");

    // split the list into lines
    let lines = contents.lines();

    let mut highest: u16 = 0;
    let mut sum: u16 = 0;
    let mut elf_index: u8 = 1;
    let mut highest_index: u8 = 1;

    // iterate through the lines of the list and sum the values
    for line in lines {
        // when we reach an empty line
        if line == "" {
            // check if we have a new highest sum and record it
            if sum > highest {
                highest = sum;
                highest_index = elf_index
            }

            // restart the sum
            elf_index += 1;
            sum = 0;
        } else {
            // when not an empty line add the value to the sum
            let value: u16 = line.parse().unwrap();
            sum += value;
        }
    }

    print!("\nThe elf with the most calories was #{highest_index} with {highest} calories!\n")
}
