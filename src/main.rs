use std::io;
enum Turn {
	First,
	Second
}
struct Game {
	board: Vec<Vec<String>>,
	turn: Turn,
	scores: (u32, u32)
}
impl Game {
	fn new() -> Game {
		let mut vector = Vec::new();
		let mut index = 0;
        for _i in 0..3 {
            let mut temp_vector: Vec<String> = Vec::new();
            for _j in 0..3 {
                index += 1;
                temp_vector.push(index.to_string());
            }
            vector.push(temp_vector);
        }
		Game {
			board: vector,
			turn: Turn::First,
			scores: (0, 0)
		}
	}
	fn print(&self) {
		println!("Tic Tac Toe World");
        for i in 0..self.board.len() {
            let mut string = String::new();
            for j in 0..self.board[i].len() {
                string.push_str(&self.board[i][j].to_string());
                string.push_str(" ");
            }
            println!("{}", string);
        }
	}
	fn clear(&mut self) {
		let mut vector = Vec::new();
		let mut index = 0;
        for _i in 0..3 {
            let mut temp_vector: Vec<String> = Vec::new();
            for _j in 0..3 {
                index += 1;
                temp_vector.push(index.to_string());
            }
            vector.push(temp_vector);
        }
       self.board = vector;
	}
	fn full(&self) -> bool {
		for i in 0..self.board.len() {
			for j in 0..self.board[i].len() {
				if self.board[i][j] != "X" && self.board[i][j] != "O" {
					return false
				}
			}
		}
		return true
	}
	fn won(&self) -> String {
		// horizontal
		for i in 0..self.board.len() {
			if self.board[i][0] == self.board[i][1] && self.board[i][1] == self.board[i][2] {
				return self.board[i][0].to_string()
			}
		}
		// vertical
		for i in 0..self.board.len() {
			if self.board[0][i] == self.board[1][i] && self.board[1][i] == self.board[2][i] {
				return self.board[0][i].to_string()
			}
		}
		// diagonal
		if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
			return self.board[0][0].to_string()
		}
		if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
			return self.board[0][2].to_string()
		}
		return String::from("none");
	}
	fn occupy(&mut self, number: &String) {
		for i in 0..self.board.len() {
			for j in 0..self.board[i].len() {
				if self.board[i][j] == number.to_string() {
					self.board[i][j] = match self.turn {
						Turn::First => String::from("X"),
						Turn::Second => String::from("O"),
					};
					self.turn = match self.turn {
						Turn::First => Turn::Second,
						Turn::Second => Turn::First,
					};
				}
			}
		}
		if self.full() {
			println!("The Board is Full!");
			self.clear();
		}
		let result = self.won();
		if result.as_str() != "none" {
			match result.as_str() {
				"X" => self.scores.0 += 1,
				"O" => self.scores.1 += 1,
				 _ => ()
			}
			println!("{} has won!", result);
			self.print();
			self.clear();
		}
	}
}
fn main() {
	 let mut game = Game::new();
	 loop {
	 	game.print();
	 	println!("It is {}'s Turn                             (scores) X = {},  O = {}", match game.turn {
	 		Turn::First => "X",
	 		Turn::Second => "O"
	 	}, game.scores.0, game.scores.1);
	 	let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
       let values: Vec<&str> = guess.split_whitespace().collect();
       if values.len() == 0 {
       	continue;
       }
       game.occupy(&String::from(values[0]));
	 }
	}
