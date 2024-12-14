// Worry about printing and manipulating the grid. The should be done differently with address and pointers
// Error handling to code
// Print things to go in spirals?
// Take in user input to switch spiral direction?

/*
- Grid should have an array of elements
- Each element has a position and "thing".
- Thing should have symbol and etc..
-
*/
use std::{fmt, thread, time};

#[derive(Clone, Copy)]
enum Tile {
    Player,
    Ground,
    Diamond,
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Player => write!(f, "P"),
            Tile::Ground => write!(f, "."),
            Tile::Diamond => write!(f, "D"),
        }
    }
}

struct Grid {
    tiles: [[Tile; 10]; 10],
    height: i32,
    width: i32,
}

impl Grid {
    fn new() -> Self {
        let mut grid = Grid {
            tiles: [[Tile::Ground; 10]; 10],
            height: 10,
            width: 10,
        };

        grid.tiles[0][0] = Tile::Player;
        grid.tiles[9][9] = Tile::Diamond;

        grid
    }

    fn move_player(&mut self, width: usize, height: usize) {
        self.tiles[width][height] = Tile::Player;
    }

    fn get(&self, width: usize, height: usize) -> Tile {
        self.tiles[width][height]
    }

    fn display(&self) {
        for width in &self.tiles {
            for tile in width {
                print!("{} ", tile);
            }
            println!("");
        }
    }
}

fn start() {
    println!("Starting Application!");

    let grid: Grid = Grid::new();

    grid.display();
    // let mut bullet: Bullet = build_bullet();

    // loop {
    //     wait();
    //     update_grid(&grid, &mut bullet);
    //     print_line(&grid, &bullet);
    // }
}

fn main() {
    start();
}
