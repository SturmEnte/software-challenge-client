pub struct Board {
	pub board: [[i8; 16]; 8],
	pub initialized: bool,
}

impl Board {
	pub fn new() -> Board {
		Board { board: [[0; 16]; 8], initialized: false }
	}

	pub fn get_field(&self, x: usize, y: usize) -> i8 {
		self.board[y][x]
	}

	pub fn set_field(&mut self, x: usize, y: usize, value: i8) {
		self.board[y][x] = value;
	}

	pub fn print(&self) {
		let mut i = 0;
		for _y in self.board {
			print!("Y {} |", i + 1);
			for x in self.board[i] {
				let v: String;
				if x == 0 {
					v = String::from("X");
				} else if x == -1 {
					v = String::from("λ");
				} else if x == -2 {
					v = String::from("ω");
				} else {
					v = x.to_string();
				}
				print!(" {v}");
			}
			print!("\n");
			i += 1;
		}
	}
}