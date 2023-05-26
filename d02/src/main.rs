use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    println!("\nAdvent of Code - Day 2\n");

    // encrypted strategy guide input
    // first column is what opponent is going to play
    // A Rock
    // B Paper
    // C Scissors
    // second column is what should play in response
    // X Rock
    // Y Paper
    // Z Scissors

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let strat = fs::read_to_string(file_path).expect("Expected to read file");

    println!("Strategy loaded:\n{strat}");

    // Scores
    // Rock 1
    // Paper 2
    // Scissors 3
    // Lose 0
    // Draw 3
    // Win 6
    let mut shape_points = HashMap::new();
    shape_points.insert('X', 1);
    shape_points.insert('Y', 2);
    shape_points.insert('Z', 3);

    let mut points: u8 = 0;
    for line in strat.lines() {
        // points for result
        if line == "A Y" || line == "B Z" || line == "C X" { // win
            points += 6;
        } else if line == "A X" || line == "B Y" || line == "C Z" { // draw
            points += 3;
        }

        // points for shape
        let shape = line.chars().nth(2).unwrap();
        points += shape_points.get(&shape).copied().unwrap_or(0);
    }

    println!("Strategy points: {points}");
}