//use std::collections::btree_map::Values;
use std::sync::{Mutex, MutexGuard};
use std::str::from_utf8;

use quick_xml::Reader;
use quick_xml::events::Event;

use crate::GameData;
use crate::game_move::Move;

pub fn parse_memento (message: &[u8], game_data: &Mutex<GameData>) {
    let mut game_data: MutexGuard<GameData> = game_data.lock().unwrap();

    let mut reader: Reader<&[u8]> = Reader::from_bytes(&message);
    reader.trim_text(true);
    reader.expand_empty_elements(true);

    let mut buf: Vec<u8> = Vec::new();

    let mut first_y: bool = true;
    let mut first_x: bool = true;
    let mut y: usize = 0;
    let mut x: usize = 0;

    let mut i: i32 = -1;

    let mut mv: Move = Move {from_x: -1, from_y: -1, to_x: -1, to_y: -1};

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    //this is now done in game_data.apply_move()
                    //b"state" => {
                    //    turn = String::from_utf8(e.try_get_attribute("turn").unwrap().unwrap().value.to_vec()).unwrap().parse::<i8>().unwrap();
                    //    game_data.turn = turn;
                    //},
                    b"list" => {
                        if !first_y {
                            y += 1;
                        } else {
                            first_y = false;
                        }
                        
                        if y % 2 == 0 {
                            x = 0;  
                        } else {
                            x = 1
                        }

                        first_x = true;
                    },
                    b"field" => {
                        if !first_x {
                            x += 2;
                        } else {
                            first_x = false;
                        }
                    },
                    b"from" => {
                        let x: i8 = String::from_utf8(e.try_get_attribute("x").unwrap().unwrap().value.to_vec()).unwrap().parse::<i8>().unwrap();
                        let y: i8 = String::from_utf8(e.try_get_attribute("y").unwrap().unwrap().value.to_vec()).unwrap().parse::<i8>().unwrap();
                        mv.from_x = x;
                        mv.from_y = y;
                    },
                    b"to" => {
                        let x: i8 = String::from_utf8(e.try_get_attribute("x").unwrap().unwrap().value.to_vec()).unwrap().parse::<i8>().unwrap();
                        let y: i8 = String::from_utf8(e.try_get_attribute("y").unwrap().unwrap().value.to_vec()).unwrap().parse::<i8>().unwrap();
                        mv.to_x = x;
                        mv.to_y = y;
                        game_data.apply_move(&mv);
                    }
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => {
                if !game_data.board.initialized {
                    let txt: String = from_utf8(e.unescaped().unwrap().into_owned().as_slice()).unwrap().to_string();

                    if i == -1 {
                        game_data.set_start_team(&from_utf8(e.unescaped().unwrap().into_owned().as_slice()).unwrap().to_string());
                    } else if i >= 0 && i < 64 {
                        let field: i8;

                        if txt == "ONE" {
                            field = -1
                        } else if txt == "TWO" {
                            field = -2
                        } else {
                            field = txt.parse::<i8>().unwrap();
                        }

                        game_data.board.set_field(x, y, field);
                    }
                    
                    i += 1;
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), 
        }
        buf.clear();
    }
    
    game_data.board.initialized = true;
    game_data.board.print();
}