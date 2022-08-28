use crate::board::Board;

pub struct GameData {
    pub board: Board,
    pub start_team: i8,
    pub team: i8,
    pub opponent: i8,
}

impl GameData {
    pub fn new() -> GameData {
        GameData { team: 0, board: Board::new(), start_team: 0, opponent: 0 }
    }
    
    pub fn set_team(&mut self, team: &String) {
        if team == "ONE" {
            self.team = 1;
            self.opponent = 2;
        } else {
            self.opponent = 1;
            self.team = 2;
        }
    }

    pub fn set_start_team(&mut self, team: &String) {
        if team == "ONE" {
            self.start_team = 1;
        } else {
            self.start_team = 2;
        }
    }
}