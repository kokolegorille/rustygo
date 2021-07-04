extern crate rustygo;

use rustygo::game::{Game, GameStatus};
use rustygo::board::{Point};

use std::io;

fn main() {
    let mut game = Game::new(19);

    while game.status != GameStatus::Finished {
        let mut input = String::new();
        println!("Enter move: [x y, <P>ass, or <R>esign]");

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

                        match game.play(Point(x, y)) {
                            Ok(play_move) => println!("Ok {:?}", play_move),
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                }

                println!("{}", game.to_string());
            },
            Err(e) => println!("Something went wrong: {} {}", e, input),
        }
    }
}
