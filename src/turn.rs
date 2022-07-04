
enum TURN {
    X,
    O,
}

pub struct Turn {
    state: TURN,
    pub char: char,
}

impl Turn {
    pub fn new() -> Turn {
        Turn {
            state: TURN::X,
            char: 'X',
        }
    }

    pub fn swap(&mut self) {
        match self.state {
            TURN::X => {
                self.char = 'O';
                self.state = TURN::O;
            }
            TURN::O => {
                self.char = 'X';
                self.state = TURN::X;
            }
        }
    }
}
