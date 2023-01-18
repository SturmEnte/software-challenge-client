use crate::board::Board;
use crate::game_move::Move;

pub struct GameData {
    pub board: Board,
    pub start_team: i8,
    pub team: i8,
    pub opponent: i8,
    pub turn: i8,
    pub room_id: String,
    pub fishes_team: i8,
    pub fishes_opponent: i8
}

impl GameData {
    pub fn new() -> GameData {
        GameData { team: 0, board: Board::new(), start_team: 0, opponent: 0, turn: 1, room_id: String::new(), fishes_team: 0, fishes_opponent: 0 } //tuen 1, because in turn 0 apply_move is not executed
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

    pub fn apply_move(&mut self, mv: &Move) {
        let mut from_team: i8 = 0 - self.team;
        println!("Turn {}", self.turn);
        if self.turn <= 8 {
            if self.start_team == self.team {
                if self.turn % 2 == 0 {
                    from_team = 0 - self.opponent;
                }
            } else {
                if self.turn % 2 != 0 {
                    from_team = 0 - self.opponent;
                }
            }
        }
        else {
            from_team = self.board.get_field(mv.from_x as usize, mv.from_y as usize);
            self.board.set_field(mv.from_x as usize, mv.from_y as usize, 0);
        }

        if from_team == 0 - self.team {
            self.fishes_team += self.board.get_field(mv.to_x as usize, mv.to_y as usize);
        }
        else {
            self.fishes_opponent += self.board.get_field(mv.to_x as usize, mv.to_y as usize);
        }
        println!("Fishes: We: {}, Opponent: {}", self.fishes_team, self.fishes_opponent);

        self.board.set_field(mv.to_x as usize, mv.to_y as usize, from_team);

        self.turn += 1;
	}
}