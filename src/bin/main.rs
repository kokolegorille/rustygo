extern crate go;

use go::game::{Game, GameStatus};
use go::board::{Point};

use std::io;

fn main() {
    let mut game = Game::new(19);

    while game.status != GameStatus::Finished {
        let mut input = String::new();
        println!("Enter move: (x y separated by a space, or P, or R)");

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match &input.trim() {
                    &"P" | &"p" => {
                        if let Ok(play_move) = game.pass() { 
                            println!("Ok {:?}", play_move) 
                        }
                    },
                    &"R" | &"r" => {
                        if let Ok(play_move) = game.resign() { 
                            println!("Ok {:?}", play_move) 
                        }
                    },
                    _ => {
                        let i: Vec<&str> = input.split_whitespace().collect();

                        if i.len() != 2 {
                            continue;
                        }

                        let x = i[0]
                            .trim()
                            .parse::<usize>()
                            .unwrap();
                        let y = i[1]
                            .trim()
                            .parse::<usize>()
                            .unwrap();
        
                        // Plays
                        match game.play(Point(x, y)) {
                            Ok(play_move) => println!("Ok {:?}", play_move),
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                }

                // println!("{:?}", game);

                println!("Current Turn: {:?}", game.current_turn);
                println!("Move Count: {}", game.move_count);
                println!("Game Status: {:?}", game.status);
                println!("{}", game.board.to_string());
            },
            Err(e) => println!("Something went wrong: {} {}", e, input),
        }
    }
}

// extern crate go;

// use go::board::{Board, Color, Point};

// fn main() {
//     let mut board = Board::new(19);
//     // println!("{:?}", board);

//     // let neighbors = board.neighbors(Point(2, 2));
//     // println!("{:?}", neighbors);

//     // match board.place_stone(Color::Black, Point(2, 2)) {
//     //     Ok(_) => println!("Ok"),
//     //     Err(e) => println!("Error: {}", e),
//     // }

//     // println!("{}", board.to_string());

//     // println!("{:?}", board);



//     // match board.place_stone(Color::Black, Point(3, 2)) {
//     //     Ok(_) => println!("Ok"),
//     //     Err(e) => println!("Error: {}", e),
//     // }

//     // println!("{}", board.to_string());

//     // println!("{:?}", board);

//     // match board.place_stone(Color::White, Point(1, 2)) {
//     //     Ok(_) => println!("Ok"),
//     //     Err(e) => println!("Error: {}", e),
//     // }

//     // match board.place_stone(Color::White, Point(2, 3)) {
//     //     Ok(_) => println!("Ok"),
//     //     Err(e) => println!("Error: {}", e),
//     // }

//     // match board.place_stone(Color::White, Point(3, 3)) {
//     //     Ok(_) => println!("Ok"),
//     //     Err(e) => println!("Error: {}", e),
//     // }

//     // match board.place_stone(Color::White, Point(4, 2)) {
//     //     Ok(_) => println!("Ok"),
//     //     Err(e) => println!("Error: {}", e),
//     // }

//     // match board.place_stone(Color::White, Point(3, 1)) {
//     //     Ok(_) => println!("Ok"),
//     //     Err(e) => println!("Error: {}", e),
//     // }

//     // match board.place_stone(Color::White, Point(2, 1)) {
//     //     Ok(_) => println!("Ok"),
//     //     Err(e) => println!("Error: {}", e),
//     // }

//     // println!("{}", board.to_string());

//     // println!("{:?}", board);

//     match board.place_stone(Color::White, Point(18, 18)) {
//         Ok(_) => println!("Ok"),
//         Err(e) => println!("Error: {}", e),
//     }

//     match board.place_stone(Color::Black, Point(18, 17)) {
//         Ok(_) => println!("Ok"),
//         Err(e) => println!("Error: {}", e),
//     }

//     match board.place_stone(Color::White, Point(17, 18)) {
//         Ok(_) => println!("Ok"),
//         Err(e) => println!("Error: {}", e),
//     }

//     match board.place_stone(Color::Black, Point(16, 17)) {
//         Ok(_) => println!("Ok"),
//         Err(e) => println!("Error: {}", e),
//     }

//     match board.place_stone(Color::White, Point(17, 17)) {
//         Ok(_) => println!("Ok"),
//         Err(e) => println!("Error: {}", e),
//     }

//     match board.place_stone(Color::Black, Point(17, 16)) {
//         Ok(_) => println!("Ok"),
//         Err(e) => println!("Error: {}", e),
//     }

//     match board.place_stone(Color::White, Point(16, 18)) {
//         Ok(_) => println!("Ok"),
//         Err(e) => println!("Error: {}", e),
//     }

//     match board.place_stone(Color::Black, Point(15, 18)) {
//         Ok(_) => println!("Ok"),
//         Err(e) => println!("Error: {}", e),
//     }



//     println!("{}", board.to_string());

//     println!("{:?}", board);
// }