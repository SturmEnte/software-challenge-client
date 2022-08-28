//use std::collections::btree_map::Values;
use std::sync::Mutex;
use std::str::from_utf8;

use quick_xml::Reader;
use quick_xml::events::Event;

use crate::GameData;

pub fn parse_memento (message: &[u8], game_data: &Mutex<GameData>) {
    let mut game_data = game_data.lock().unwrap();

    let mut reader = Reader::from_bytes(&message);
    reader.trim_text(true);
    reader.expand_empty_elements(true);

    let mut buf = Vec::new();

    let mut first_y = true;
    let mut first_x = true;
    let mut y = 0;
    let mut x = 0;

    let mut i = -1;

    let mut turn: i8 = 0;

    let mut from_team: i8 = 0 - game_data.team; //later changed if opponent made move

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"state" => {
                        turn = String::from_utf8(e.try_get_attribute("turn").unwrap().unwrap().value.to_vec()).unwrap().parse::<i8>().unwrap();
                        game_data.turn = turn;
                    },
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
                        let x = String::from_utf8(e.try_get_attribute("x").unwrap().unwrap().value.to_vec()).unwrap().parse::<usize>().unwrap();
                        let y = String::from_utf8(e.try_get_attribute("y").unwrap().unwrap().value.to_vec()).unwrap().parse::<usize>().unwrap();
                        from_team = game_data.board.get_field(x, y);
                        game_data.board.set_field(x, y, 0);
                    },
                    b"to" => {
                        let x = String::from_utf8(e.try_get_attribute("x").unwrap().unwrap().value.to_vec()).unwrap().parse::<usize>().unwrap();
                        let y = String::from_utf8(e.try_get_attribute("y").unwrap().unwrap().value.to_vec()).unwrap().parse::<usize>().unwrap();

                        if turn <= 8 {
                            if game_data.start_team == game_data.team {
                                if turn % 2 == 0 {
                                    from_team = 0 - game_data.opponent.to_owned();
                                }
                            } else {
                                if turn % 2 != 0 {
                                    from_team = 0 - game_data.opponent.to_owned();
                                }
                            }
                        }
                        game_data.board.set_field(x, y, from_team);
                    }
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => {
                if !game_data.board.initialized {
                    let txt = from_utf8(e.unescaped().unwrap().into_owned().as_slice()).unwrap().to_string();

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