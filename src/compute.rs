use std::sync::Mutex;

use crate::GameData;
use crate::Move;

pub fn get_possible_moves(game_data: &Mutex<GameData>) -> Vec<Move> {
    let mut pmvs: Vec<Move> = Vec::new();
    let game_data = game_data.lock().unwrap();
    if game_data.turn <= 7 { // startmove
        game_data.board.get_same_fields(1).iter().for_each(|field| {
            pmvs.push(Move {from_x: -1, from_y: -1, to_x: field.0, to_y: field.1});
        });
    }
    else { // normal move
        let mut dest_x: i8 = 0;
        let mut dest_y: i8 = 0;
        game_data.board.get_same_fields(0-game_data.team).iter().for_each(|position| {
            [(2,0),(-2,0),(1,1),(1,-1),(-1,1),(-1,-1)].iter().for_each(|direction| {
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
                    pmvs.push(Move {from_x: position.0, from_y: position.1, to_x: dest_x, to_y: dest_y});
                }
            });
        });
    }
    return pmvs;
}

pub fn compute_move(game_data: &Mutex<GameData>) -> Move {
    let pmvs: Vec<Move> = get_possible_moves(&game_data);
    let mv = pmvs.first().unwrap();
    Move { from_x: mv.from_x, from_y: mv.from_y, to_x: mv.to_x, to_y: mv.to_y }
}