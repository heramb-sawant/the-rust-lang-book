use std::fmt::{self};

#[derive(Clone, Copy)]
pub enum Tile {
    Player,
    Ground,
    Target,
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Player => write!(f, "P"),
            Tile::Ground => write!(f, "."),
            Tile::Target => write!(f, "T"),
        }
    }
}

pub struct Grid {
    tiles: Vec<Vec<Tile>>,
    player_coordinate: (usize, usize),
    target_coordinate: (usize, usize),
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum GameError {
    OutOfBounds,
}

impl Grid {
    pub fn new(length: usize) -> Result<Self, GameError> {
        if length > 50 {
            return Err(GameError::OutOfBounds);
        }

        let mut tiles = vec![vec![Tile::Ground; length]; length];
        let player_coordinate = (0, 0);
        let target_coordinate = (length - 1, length - 1);

        tiles[player_coordinate.0][player_coordinate.1] = Tile::Player;
        tiles[target_coordinate.0][target_coordinate.1] = Tile::Target;

        Ok(Grid {
            tiles,
            player_coordinate,
            target_coordinate,
        })
    }

    pub fn move_player(&mut self, direction: &Direction) -> Option<GameError> {
        let player_coordinate = self.player_coordinate;

        match direction {
            Direction::Right => {
                if player_coordinate.1 == 9 {
                    return Some(GameError::OutOfBounds);
                }
            }
            Direction::Left => {
                if player_coordinate.1 == 0 {
                    return Some(GameError::OutOfBounds);
                }
            }
            Direction::Up => {
                if player_coordinate.0 == 9 {
                    return Some(GameError::OutOfBounds);
                }
            }
            Direction::Down => {
                if player_coordinate.0 == 0 {
                    return Some(GameError::OutOfBounds);
                }
            }
        };

        let new_coordinate = match direction {
            Direction::Right => (player_coordinate.0, player_coordinate.1 + 1),
            Direction::Left => (player_coordinate.0, player_coordinate.1 - 1),
            Direction::Up => (player_coordinate.0 + 1, player_coordinate.1),
            Direction::Down => (player_coordinate.0 - 1, player_coordinate.1),
        };

        let tile_to_move = self.tiles[new_coordinate.0][new_coordinate.1];
        let player_tile = self.tiles[player_coordinate.0][player_coordinate.1];

        self.tiles[new_coordinate.0][new_coordinate.1] = player_tile;
        self.tiles[player_coordinate.0][player_coordinate.1] = tile_to_move;
        self.player_coordinate = new_coordinate;

        return None;
    }

    // fn get(&self, width: usize, height: usize) -> Tile {
    //     self.tiles[width][height]
    // }

    pub fn display(&self) {
        for rows in self.tiles.iter().rev() {
            for tile in rows.iter() {
                print!("{} ", tile);
            }
            println!("")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_generation() {
        let gird = Grid::new(3).unwrap_or(panic!("Could not create grid"));

        assert_eq!(gird, Grid);
        assert_eq!(gird[0][0], Tile::Player);
    }
}
