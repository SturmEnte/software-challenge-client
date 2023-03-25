use std::sync::Mutex;
use std::net::TcpStream;
use std::io::Write;

use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};

use crate::parse_memento::parse_memento;
use crate::GameData;
use crate::compute::compute_move;
use crate::Move;

pub fn parse_message(buffer: [u8; 5000], n: usize, game_data: &Mutex<GameData>, stream: &mut Option<&mut TcpStream>) -> bool {

    let message: &[u8] = &buffer[..n];

    let mut reader: Reader<&[u8]> = Reader::from_bytes(&message);
    reader.trim_text(true);
    reader.expand_empty_elements(true);

    let mut buf: Vec<u8> = Vec::new();
    
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"data" => {
                        let class: String = String::from_utf8(e.try_get_attribute("class").unwrap().unwrap().value.to_vec()).unwrap();

                        match class.as_str() {
                            "welcomeMessage" => welcome_message(e, &game_data),
                            "memento" => {
                                println!("Memento");
                                parse_memento(&message, &game_data);
                            },
                            "moveRequest" => {
                                println!("Move Request");
                                let mv: Move = compute_move(&game_data);
                                mv.print();
                                let send_str: String;
                                if mv.from_x == -1{
                                    send_str = format!(r#"<room roomId="{}"><data class="move"><to x="{}" y="{}"/></data></room>"#, game_data.lock().unwrap().room_id, mv.to_x, mv.to_y);
                                }
                                else {
                                    send_str = format!(r#"<room roomId="{}"><data class="move"><from x="{}" y="{}"/><to x="{}" y="{}"/></data></room>"#, game_data.lock().unwrap().room_id, mv.from_x, mv.from_y, mv.to_x, mv.to_y);
                                }
                                
                                stream.as_mut().unwrap().write(send_str.as_bytes()).unwrap();

                            },
                            "result" => {
                                println!("Result");
                                return true;
                            },
                            _ => (),
                        }
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), 
        }
        buf.clear();
    }
    false
}

fn welcome_message(e: &BytesStart, data: &Mutex<GameData>) {
    let team = String::from_utf8(e.try_get_attribute("color").unwrap().unwrap().value.to_vec()).unwrap();
    println!("Received welcome message");
    println!("Team: {}", &team);
    data.lock().unwrap().set_team(&team);
}