use std::sync::Mutex;

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

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
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
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => {
                let txt = std::str::from_utf8(e.unescaped().unwrap().into_owned().as_slice()).unwrap().to_string();

                if ii >= 0 && ii < 64 {
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
                
                ii += 1;
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), 
        }
        buf.clear();
    }

    game_data.board.print();
}