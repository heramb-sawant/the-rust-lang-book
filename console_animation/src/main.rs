// Worry about printing and manipulating the grid. The should be done differently with address and pointers
// Error handling to code
// Print things to go in spirals?
// Take in user input to switch spiral direction?

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Bullet {
    char: String,
    position: u32,
    direction: Direction,
}

struct Grid {
    space_char: String,
    length: u32,
}

fn build_grid(length: u32) -> Grid {
    let grid = Grid {
        length,
        space_char: String::from(' '),
    };

    grid
}

fn build_bullet() -> Bullet {
    let grid = Bullet {
        char: String::from('O'),
        position: 0,
        direction: Direction::Right,
    };

    grid
}

fn print_line(grid: &Grid, bullet: &Bullet) {
    let mut line = String::new();
    let mut i = 0;

    while i < grid.length {
        if i == bullet.position {
            line.push_str(&bullet.char);
        } else if i == 0 {
            line.push_str("|");
        } else if i == grid.length - 1 {
            line.push_str("|");
        } else {
            line.push_str(&grid.space_char);
        }

        i += 1;
    }

    println!("{}", line);
}

fn update_grid(grid: &Grid, bullet: &mut Bullet) {
    match bullet.direction {
        Direction::Left => {
            if bullet.position == 0 {
                bullet.direction = Direction::Right;
                bullet.position += 1;
            } else {
                bullet.position -= 1;
            }
        }
        Direction::Right => {
            if bullet.position == &grid.length - 1 {
                bullet.direction = Direction::Left;
                bullet.position -= 1;
            } else {
                bullet.position += 1;
            }
        }
    }
}

fn start() {
    println!("Starting Application!");

    let grid: Grid = build_grid(100);
    let mut bullet: Bullet = build_bullet();

    loop {
        update_grid(&grid, &mut bullet);
        print_line(&grid, &bullet);
    }
}

fn main() {
    start();
}
