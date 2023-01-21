use std::sync::Mutex;
use std::sync::MutexGuard;
use std::time::Instant;
use std::cmp::{max,min};

use crate::GameData;
use crate::Move;

pub fn get_possible_moves(game_data: &GameData, use_opponent: bool) -> Vec<Move> {
    let mut possible_moves: Vec<Move> = Vec::new();

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

    let mut requested_team: i8 = game_data.team;
    if use_opponent {
        requested_team = game_data.opponent;
    }

    game_data.board.get_same_fields(0-requested_team).iter().for_each(|position: &(i8, i8)| {
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

pub fn minimax(game_data: &GameData, depth: i8, maximizing_player: bool) -> i32 {
    if depth == 0 || game_data.game_over {
        return game_data.static_evaluation();
    }
    if maximizing_player {
        let mut max_eval: i32 = -2147483648; //minimum i32 value
        for mv in get_possible_moves(game_data, false) {
            let mut new_game_data = game_data.copy();
            new_game_data.apply_move(&mv);
            let eval: i32 = minimax(&new_game_data,depth-1,false);
            max_eval = max(max_eval, eval);
        }
        return max_eval;
    }
    else {
        let mut min_eval: i32 = 2147483647; //maximum i32 value
        for mv in get_possible_moves(game_data, true) {
            let mut new_game_data: GameData = game_data.copy();
            new_game_data.apply_move(&mv);
            let eval: i32 = minimax(game_data, depth-1, true);
            min_eval = min(min_eval, eval);
        }
        return min_eval;
    }
}

pub fn compute_move(game_data: &Mutex<GameData>) -> Move {
    let minimax_depth: i8 = 2;
    let game_data: MutexGuard<GameData> = game_data.lock().unwrap();
    let max_time: u128 = 1900; //u128 for compatibility with start_time.elapsed()
    let start_time = Instant::now();
    let possible_moves: Vec<Move> = get_possible_moves(&game_data, false);
    let mut rated_moves: Vec<(Move, i32)> = Vec::new();
    for mv in possible_moves {
        let mut new_game_data: GameData = game_data.copy();
        new_game_data.apply_move(&mv);
        let rating: i32 = minimax(&new_game_data, minimax_depth, true);
        mv.print();
        println!("rating: {}", rating);
        rated_moves.push((mv, rating));
    }
    println!("end---");
    let mut the_mv: Move = Move::new();
    let mut the_mv_rating: i32 = -2147483648; //minimum i32 value
    for rated_mv in rated_moves {
        if rated_mv.1 > the_mv_rating {
            the_mv = rated_mv.0;
            the_mv_rating = rated_mv.1;
        }
    }
    /*loop {
        println!("running");
        if start_time.elapsed().as_millis() >= max_time {
            println!("done");
            break;
        }        
    }*/
    Move { from_x: the_mv.from_x, from_y: the_mv.from_y, to_x: the_mv.to_x, to_y: the_mv.to_y }
}