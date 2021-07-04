use super::board::{Board, Color, Point};

#[derive(Debug, PartialEq)]
pub enum GameStatus {
    Playing,
    Finished,
}

#[derive(Debug)]
pub enum Move {
    Pass(Color),
    Resign(Color),
    Play(Color, Point),
}

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    pub current_turn: Color,
    pub move_count: usize,
    pub status: GameStatus,
    pub winner: Option<Color>,
    consecutive_passes: u8,
}

impl Game {
    pub fn new(size: usize) -> Self {
        Self {
            board: Board::new(size),
            current_turn: Color::Black,
            move_count: 0,
            status: GameStatus::Playing,
            winner: None,
            consecutive_passes: 0,
        }
    }

    pub fn play(&mut self, point: Point) -> Result<Move, &'static str> {
        let play_move = Move::Play(self.current_turn, point);
        match self.board.place_stone(self.current_turn, point) {
            Ok(_) => {
                // If move is valid... place stone on board
                self.move_count += 1;
                self.consecutive_passes = 0;
                self.current_turn = self.current_turn.next();

                Ok(play_move)
            },
            Err(e) => Err(e)
        }
    }

    pub fn pass(&mut self) -> Result<Move, &'static str> {
        self.move_count += 1;
        self.consecutive_passes += 1;
        // persists initial current_turn
        let current_turn = self.current_turn;
        self.current_turn = self.current_turn.next();
        if self.consecutive_passes >= 2 {
            self.status = GameStatus::Finished;
            // Calculate who the winner is...
        }
        Ok(Move::Pass(current_turn))
    }

    pub fn resign(&mut self) -> Result<Move, &'static str> {
        self.status = GameStatus::Finished;
        self.winner = Some(self.current_turn.next());
        Ok(Move::Resign(self.current_turn))
    }
}

impl ToString for Game {
    fn to_string(&self) -> String {
        let mut string = String::new();
        string.push_str(&format!("Current Turn: {:?}\n", self.current_turn));
        string.push_str(&format!("Move Count: {:?}\n", self.move_count));
        string.push_str(&format!("Game Status: {:?}\n", self.status));
        string.push_str(&format!("Captures: W {}, B {}\n", self.board.white_captures, self.board.black_captures));
        string.push_str(&format!("{}", self.board.to_string()));
        string
    }
}
