use crate::point::Point;

pub struct Board {
    width: usize,
    height: usize,
    board: Vec<char>,
}

impl Board {
    pub fn new() -> Board {Board {
            width: 3,
            height: 3,
            board: vec!['.'; 3 * 3],
        }
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.board[self.width * y + x]);
            }
            println!();
        }
    }

    pub fn place_char(&mut self, p: Point, player: char) -> bool {
        if (0..=3).contains(&p.x) && (0..=3).contains(&p.y) {
            if self.board[self.width * (p.y - 1) + (p.x - 1)] != '.' {
                println!("Pice all ready there");
                return false;
            }
            self.board[self.width * (p.y - 1) + (p.x - 1)] = player;
            true
        } else {
            println!("out of range");
            false
        }
    }

    pub fn check_win(&self, player: char) -> bool {
        // check vertical
        for x in 0..self.width {
            for y in 0..self.height {
                if self.get_board_cord(x, y) != player {
                    break;
                } else if y == self.width - 1 {
                    return true;
                }
            }
        }

        // check horzontial
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get_board_cord(x, y) != player {
                    break;
                } else if x == self.height - 1 {
                    return true;
                }
            }
        }

        // check dirangnal
        // [1,1] [2,2] [3,3]
        for i in 0..self.height {
            if self.get_board_cord(i, i) != player {
                break;
            } else if i == self.height - 1 {
                return true;
            }
        }
        // [1,3] [2,2] [3,1]
        let mut y = self.width - 1;
        for x in 0..self.height {
            if self.get_board_cord(x, y) != player {
                break;
            } else if x == self.width - 1 {
                return true;
            }
            y -= 1;
        }

        false
    }

    fn get_board_cord(&self, x: usize, y: usize) -> char {
        self.board[self.width * y + x]
    }
}
