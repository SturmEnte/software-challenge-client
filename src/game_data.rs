pub struct GameData {
    team: u8,
}

impl GameData {
    pub fn new() -> GameData {
        GameData { team: 0 }
    }

    pub fn set_team(&mut self, team: &String) {
        if team == "ONE" {
            self.team = 1;
        } else {
            self.team = 2;
        }
    }
}