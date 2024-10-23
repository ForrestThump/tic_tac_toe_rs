use std::cell::Cell;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellState {
    Player(Player),
    Empty,
}

struct Board {
    size: u8,
    board: Vec<Vec<CellState>>,
}

impl Board {
    fn new(size: Option<u8>) -> Board {
        let size: u8 = size.unwrap_or(3);

        Board {
            size: size,
            board: vec![vec![CellState::Empty; size as usize]; size as usize],
        }
    }

    fn get_cell(&self, mut coords: (u8, u8)) -> Option<CellState> {
        if coords.0 == 0
            || coords.0 > self.board.len() as u8
            || coords.1 == 0
            || coords.1 > self.board.len() as u8
        {
            return None;
        }

        coords.0 -= 1;
        coords.1 -= 1;

        Some(self.board[coords.0 as usize][coords.1 as usize])
    }

    fn set_cell(&mut self, mut coords: (u8, u8), state: CellState) {
        coords.0 -= 1;
        coords.1 -= 1;

        if coords.0 > self.size || coords.1 > self.size {
            return;
        }

        self.board[coords.0 as usize][coords.1 as usize] = state;
    }

    fn clear_board(&mut self) {
        *self = Board::new(Some(self.size));
    }

    fn print_board(&self) {
        print!("  ");
        for i in 1..self.board.len() + 1 {
            print!("{i} ");
        }
        println!("");

        for (row_index, row) in self.board.iter().enumerate() {
            let temp: usize = row_index + 1;
            print!("{temp} ");
            for (col_index, cell) in row.iter().enumerate() {
                match cell {
                    CellState::Empty => {
                        print!(" ");
                    }
                    CellState::Player(player) => match player {
                        Player::X => {
                            print!("X");
                        }
                        Player::O => {
                            print!("O");
                        }
                    },
                }
                if col_index < row.len() - 1 {
                    print!("|");
                } else {
                    println!("")
                }
            }
            if row_index < self.board.len() - 1 {
                print!("  ");
                for i in 0..self.board.len() {
                    print!("-");
                    if i < self.board.len() - 1 {
                        print!("-");
                    }
                }
                println!("");
            }
        }
    }

    fn check_board_win(&self, player: Player) -> bool {
        for i in 0..self.board.len() {
            if self.board[i]
                .iter()
                .all(|&cell| cell == CellState::Player(player))
            {
                return true;
            }

            let mut vertical_found: bool = true;
            for j in 0..self.board.len() {
                if self.board[i][j] != CellState::Player(player) {
                    vertical_found = false;
                    break;
                }
            }

            if vertical_found {
                return true;
            }
        }

        let temp_size: usize = self.board.len();
        let half_temp_size: usize = self.board.len() / 2;

        if temp_size % 2 != 0
            && self.board[half_temp_size + 1][half_temp_size + 1] == CellState::Player(player)
        {
            let mut temp_cell: (usize, usize) = (0 as usize, 0 as usize);
            let mut diagonal_found: bool = true;
            loop {
                if self.board[temp_cell.0][temp_cell.1] != CellState::Player(player) {
                    diagonal_found = false;
                    break;
                }
                temp_cell.0 += 1;
                temp_cell.1 += 1;
                if temp_cell.0 == self.board.len() {
                    break;
                }
            }

            if diagonal_found {
                return true;
            }

            temp_cell = (0 as usize, (self.board.len() - 1) as usize);

            loop {
                if self.board[temp_cell.0][temp_cell.1] != CellState::Player(player) {
                    diagonal_found = false;
                    break;
                }

                temp_cell.0 += 1;
                temp_cell.1 -= 1;
                if temp_cell.0 == self.board.len() {
                    break;
                }
            }

            if diagonal_found {
                return true;
            }
        }

        return false;
    }

    fn get_board_score(&self) -> GameState {
        if self.check_board_win(Player::X) {
            return GameState::XWon;
        } else if self.check_board_win(Player::O) {
            return GameState::OWon;
        }

        for i in 0..self.board.len() {
            if self.board[i].iter().any(|&cell| cell == CellState::Empty) {
                return GameState::Running;
            }
        }

        return GameState::Tied;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

enum GameState {
    Running,
    Tied,
    XWon,
    OWon,
}

struct Game {
    game_state: GameState,
    board: Board,
    current_player: Player,
    human_player: Player,
}

impl Game {
    fn new(size: Option<u8>) -> Game {
        Game {
            game_state: GameState::Running,
            board: Board::new(size),
            current_player: Player::X,
            human_player: Player::X,
        }
    }

    fn run(&mut self) -> () {
        loop {
            // if self.current_player == self.human_player {
            if true {
                loop {
                    if let Some((x, y)) = self.get_player_move() {
                        if let Some(state) = self.board.get_cell((x, y)) {
                            if state == CellState::Empty {
                                self.board
                                    .set_cell((x, y), CellState::Player(self.current_player));
                                break;
                            }
                        }
                    }
                    println!("Invalid input.");
                    println!("");
                }
            }
            let mut running: bool = false;
            match self.board.get_board_score() {
                GameState::Running => {
                    running = true;
                }
                GameState::XWon => {
                    println!("X won!");
                }
                GameState::OWon => {
                    println!("O win!");
                }
                GameState::Tied => {
                    println!("Shiloh won!");
                }
            }

            if !running {
                self.board.print_board();
                return;
            }

            self.switch_current_player();
        }
    }

    fn switch_current_player(&mut self) {
        match self.current_player {
            Player::X => self.current_player = Player::O,
            Player::O => self.current_player = Player::X,
        }
    }

    fn get_player_move(&self) -> Option<(u8, u8)> {
        match self.current_player {
            Player::X => {
                print!("X")
            }
            Player::O => {
                print!("O")
            }
        }
        println!("'s turn!");
        self.board.print_board();
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error: failed to read line.");

        let mut numbers: Vec<u8> = guess
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        if numbers.len() < 2 {
            return None;
        }

        Some((numbers[0], numbers[1]))
    }
}

fn main() {
    let mut game = Game::new(None);
    game.run();
}
