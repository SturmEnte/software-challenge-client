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
    pub fishes_opponent: i8,
    pub team_blocked: bool,
    pub opponent_blocked: bool,
    pub game_over: bool
}

impl GameData {
    pub fn new() -> GameData {
        GameData { team: 0, board: Board::new(), start_team: 0, opponent: 0, turn: 1, room_id: String::new(), fishes_team: 0, fishes_opponent: 0, team_blocked: false, opponent_blocked: false, game_over: false } //turn 1, because in turn 0 apply_move is not executed
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

        self.board.set_field(mv.to_x as usize, mv.to_y as usize, from_team);

        if !self.team_blocked {
            if !self.can_move(false) {
                self.team_blocked = true;
            }
        }

        if !self.opponent_blocked {
            if !self.can_move(true) {
                self.opponent_blocked = true;
            }
        }

        if self.team_blocked || self.opponent_blocked {
            self.game_over = true;
        }

        self.turn += 1;
	}

    pub fn static_evaluation(&self) -> i32 {
        //rates the current game state and returns an i32 number
        return (self.fishes_team-self.fishes_opponent) as i32;
    }

    pub fn copy(&self) -> GameData {
        GameData { board: self.board, start_team: self.start_team, team: self.team, opponent: self.opponent, turn: self.turn, room_id: self.room_id.clone(), fishes_team: self.fishes_team, fishes_opponent: self.fishes_opponent, team_blocked: self.team_blocked, opponent_blocked: self.opponent_blocked, game_over: self.game_over }
    }

    fn can_move(&self, use_opponent: bool) -> bool {
    
        // Start move (always able to move)
        if self.turn <= 8 {
            return true;
        }
    
        // Normal move
        let mut requested_team: i8 = self.team;
        if use_opponent {
            requested_team = self.opponent;
        }
    
        for position in self.board.get_same_fields(0-requested_team).iter() {
            // Check for possible moves in every direction
            // The tuples are all possible directions
            for direction in [(2,0),(-2,0),(1,1),(1,-1),(-1,1),(-1,-1)].iter() {
                if direction.0+position.0 > 15 || direction.1+position.1 > 7 || direction.0+position.0 < 0 || direction.1+position.1 < 0 {
                    continue;
                }

                if self.board.get_field((direction.0+position.0) as usize, (direction.1+position.1) as usize) < 1 {
                    continue;
                }

                return true;
            };
        };
    
        return false;
    }
}