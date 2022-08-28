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

	pub fn get_same_fields(&self, content: i8) -> Vec<(i8, i8)> {
		let mut same_fields: Vec<(i8, i8)> = Vec::new();
		let mut x: i8 = 0;
		let mut y: i8 = 0;
		self.board.iter().for_each(|line| {
			line.iter().for_each(|field| {
				if field == &content {
					same_fields.push((x, y));
				}
				x += 1;
			});
			y += 1;
			x = 0;
		});
		return same_fields;
	}

	pub fn print(&self) {
		let mut i = 0;
		for _y in self.board {
			print!("Y {} |", i + 1);
			for x in self.board[i] {
				let v: String;
				if x == 0 {
					v = String::from(".");
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