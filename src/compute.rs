use std::sync::Mutex;
use std::sync::MutexGuard;
use std::time::Instant;

use crate::GameData;
use crate::Move;

pub fn get_possible_moves(game_data: &Mutex<GameData>) -> Vec<Move> {
    let mut possible_moves: Vec<Move> = Vec::new();
    let game_data: MutexGuard<GameData> = game_data.lock().unwrap();

    // Start move
    if game_data.turn <= 8 {
        game_data.board.get_same_fields(1).iter().for_each(|field: &(i8, i8)| {
            possible_moves.push(Move {from_x: -1, from_y: -1, to_x: field.0, to_y: field.1});
        });

        return possible_moves;
    }

    // Normal move
    let mut dest_x: i8 = 0;
    let mut dest_y: i8 = 0;

    game_data.board.get_same_fields(0-game_data.team).iter().for_each(|position: &(i8, i8)| {
        // Check for possible moves in every direction
        // The tuples are all possible directions
        [(2,0),(-2,0),(1,1),(1,-1),(-1,1),(-1,-1)].iter().for_each(|direction: &(i8, i8)| {
            dest_x = position.0;
            dest_y = position.1;

            loop {
                dest_x += direction.0;
                dest_y += direction.1;

                if dest_x > 15 || dest_y > 7 || dest_x < 0 || dest_y < 0 {
                    break
                }

                if game_data.board.get_field(dest_x as usize, dest_y as usize) < 1 {
                    break
                }

                possible_moves.push(Move {from_x: position.0, from_y: position.1, to_x: dest_x, to_y: dest_y});
            }
        });
    });

    return possible_moves;
}

pub fn compute_move(game_data: &Mutex<GameData>) -> Move {
    let max_time: u128 = 1900;
    let start_time = Instant::now();
    let possible_moves: Vec<Move> = get_possible_moves(&game_data);
    let moves: &Move = possible_moves.first().unwrap();
    /*loop {
        println!("running");
        if start_time.elapsed().as_millis() >= max_time {
            println!("done");
            break;
        }        
    }*/
    Move { from_x: moves.from_x, from_y: moves.from_y, to_x: moves.to_x, to_y: moves.to_y }
}