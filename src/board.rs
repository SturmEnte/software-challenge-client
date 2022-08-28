pub struct Board {
	pub board: [[i8; 16]; 8],
}

impl Board {
	pub fn get_field(&self, x: usize, y: usize) -> i8 {
		self.board[y][x]
	}

	pub fn set_field(&mut self, x: usize, y: usize, value: i8) {
		self.board[y][x] = value;
	}
}