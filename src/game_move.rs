pub struct Move {
    pub from_x: i8,
    pub from_y: i8,
    pub to_x: i8,
    pub to_y: i8,
}

impl Move {
    pub fn new() -> Move {
        Move { from_x: -1, from_y: -1, to_x: -1, to_y: -1 }
    }

    pub fn print(&self) {
        if self.from_x == -1 {
            println!("To:  X: {}, Y: {}", self.to_x, self.to_y);
        }
        else {
            println!("From:  X: {}, Y: {}\nTo:  X: {}, Y: {}", self.from_x, self.from_y, self.to_x, self.to_y);
        }
    }
}