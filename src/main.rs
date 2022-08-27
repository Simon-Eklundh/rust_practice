mod project1;
use project1::guessing_game_main;

use std::io;

/**
 * A main function that runs any of the future projects
 */
fn main() {
    loop {
        println!("Please select a project to run:");
        println!("1. Guess the number");
        println!("2. grep");
        println!("3. project 3");
        println!("4. Quit");
        let mut selection = String::new();
        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");
        let selection: u32 = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match selection {
            1 => guessing_game_main::start(),
            2 => println!(
                "project 2 is minigrep, run it from the terminal: with\
            cargo run -- search_string file_name.txt"
            ),
            3 => println!("Project 3"),
            4 => break,
            _ => continue,
        }
    }
    println!("Thank you for using this program!");
}
