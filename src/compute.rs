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

pub fn minimax(game_data: &GameData, depth: i8, mut alpha: i32, mut beta: i32, maximizing_player: bool, max_time: u128, start_time: Instant) -> i32 {
    if start_time.elapsed().as_millis() >= max_time {
        return 0;
    }
    if depth == 0 || game_data.game_over {
        return game_data.static_evaluation();
    }
    if maximizing_player {
        let mut max_eval: i32 = -2147483648; //minimum i32 value
        for mv in get_possible_moves(game_data, false) {
            let mut new_game_data = game_data.copy();
            new_game_data.apply_move(&mv);
            let eval: i32 = minimax(&new_game_data,depth-1, alpha, beta, new_game_data.maximizing_player_bool, max_time, start_time);
            max_eval = max(max_eval, eval);
            alpha = max(alpha, eval);
            if beta <= alpha {
                break;
            }
        }
        return max_eval;
    }
    else {
        let mut min_eval: i32 = 2147483647; //maximum i32 value
        for mv in get_possible_moves(game_data, true) {
            let mut new_game_data: GameData = game_data.copy();
            new_game_data.apply_move(&mv);
            let eval: i32 = minimax(game_data, depth-1, alpha, beta, new_game_data.minimizing_player_bool, max_time, start_time);
            min_eval = min(min_eval, eval);
            beta = min(beta, eval);
            if beta <= alpha {
                break;
            }
        }
        return min_eval;
    }
}

pub fn compute_move(game_data: &Mutex<GameData>) -> Move {
    let mut minimax_depth: i8 = 1;
    let game_data: MutexGuard<GameData> = game_data.lock().unwrap();
    let max_time: u128 = 1900; //u128 for compatibility with start_time.elapsed()
    let start_time = Instant::now();
    let possible_moves: Vec<Move> = get_possible_moves(&game_data, false);
    let mut rated_moves: Vec<(&Move, i32)> = Vec::new();
    let mut fully_rated_moves: Vec<(&Move, i32)> = Vec::new();
    loop {
        for i in 0..possible_moves.len() {
            let mv = &possible_moves[i];
            let mut new_game_data: GameData = game_data.copy();
            new_game_data.apply_move(&mv);
            let rating: i32 = minimax(&new_game_data, minimax_depth, -2147483648, 2147483647, true, max_time, start_time);
            //mv.print();
            //println!("rating: {}", rating);
            rated_moves.push((mv, rating));
        }
        if start_time.elapsed().as_millis() >= max_time {
            println!("GAME OVER!! DEPTH: {}", minimax_depth);
            break;
        }
        fully_rated_moves = Vec::new();
        for i in 0..rated_moves.len() {
            fully_rated_moves.push(rated_moves[i])
        }
        minimax_depth += 1;
        if minimax_depth >= 10 {
            break;
        }
    }
    let mut best_mv: Move = Move::new();
    let mut best_mv_rating: i32 = -2147483648; //minimum i32 value
    for rated_mv in fully_rated_moves {
        if rated_mv.1 > best_mv_rating {
            best_mv = Move { from_x: rated_mv.0.from_x, from_y: rated_mv.0.from_y, to_x: rated_mv.0.to_x, to_y: rated_mv.0.to_y };
            best_mv_rating = rated_mv.1;
            println!("Move -----------------");
            rated_mv.0.print();
            println!("Rating: {}", rated_mv.1);
        }
    }
    println!("\nBest Move ------------");
    best_mv.print();
    println!("Rating: {}\n", best_mv_rating);

    println!("\nPOSSIBLE MOVES START -+-+-+-+-+-+-+-+-+-+-+-");
    for mv in get_possible_moves(&game_data.copy(), false) {
        println!("Move --- --- ---");
        mv.print();
    }
    println!("POSSIBLE MOVES END -+-+-+-+-+-+-+-+-+-+-+-\n");

    /*loop {
        println!("running");
        if start_time.elapsed().as_millis() >= max_time {
            println!("done");
            break;
        }        
    }*/

    Move { from_x: best_mv.from_x, from_y: best_mv.from_y, to_x: best_mv.to_x, to_y: best_mv.to_y }
}