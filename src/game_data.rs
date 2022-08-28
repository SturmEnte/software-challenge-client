use crate::board::Board;

pub struct GameData {
    pub team: u8,
    pub board: Board,
}

impl GameData {
    pub fn new() -> GameData {
        GameData { team: 0, board: Board::new() }
    }
    
    pub fn set_team(&mut self, team: &String) {
        if team == "ONE" {
            self.team = 1;
        } else {
            self.team = 2;
        }
    }
}