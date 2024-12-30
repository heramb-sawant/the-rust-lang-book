/*
- Add some simple tests
- Make movement an iterator to record number of moves like counter example?
- Add some custom traits
- Add closures
- How to add closures to this?
- How can I add generics and traits to this?
    - Maybe make the tile a generic type instead of an enum.
    - Add movement patterns to different types of players.
    - If you add an enemy you could maybe add a movable trait?
*/
use std::{collections::HashMap, io};

use console_animation::{Direction, GameError, Grid};

fn clear_page() {
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
}

fn initialize_grid() -> Grid {
    loop {
        println!("Enter grid size...");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let grid_size = match input.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Grid size is too large");
                continue;
            }
        };

        return match Grid::new(grid_size) {
            Ok(new_grid) => new_grid,
            Err(error) => match error {
                GameError::OutOfBounds => {
                    println!("Grid size is too large");
                    continue;
                }
            },
        };
    }
}

fn start() {
    println!("Starting Application!");

    let mut grid = initialize_grid();
    let movement_key_maps = HashMap::from([
        ('w', Direction::Up),
        ('s', Direction::Down),
        ('a', Direction::Left),
        ('d', Direction::Right),
    ]);

    clear_page();
    grid.display();

    loop {
        println!("Next Move... (A,W,S,D)?");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let key_pressed: char = match input.trim().parse() {
            Ok(char) => char,
            Err(_) => continue,
        };

        match movement_key_maps.get(&key_pressed) {
            Some(direction) => {
                let did_player_move = grid.move_player(direction);
                match did_player_move {
                    Some(_) => {
                        println!("Invalid input.");
                        continue;
                    }
                    None => {}
                }
            }
            None => {
                println!("Invalid input.");
                continue;
            }
        }

        clear_page();
        grid.display();
    }
}

fn main() {
    start();
}
