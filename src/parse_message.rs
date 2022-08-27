use std::sync::Mutex;

use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use crate::GameData;

pub fn parse_message(buffer: [u8; 5000], n: usize, data: &Mutex<GameData>) {

    let message = &buffer[..n];

    let mut reader = Reader::from_bytes(&message);
    reader.trim_text(true);
    reader.expand_empty_elements(true);

    let mut buf = Vec::new();
    
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"data" => {
                        let class = String::from_utf8(e.try_get_attribute("class").unwrap().unwrap().value.to_vec()).unwrap();

                        match class.as_str() {
                            "welcomeMessage" => welcome_message(e, &data),
                            "memento" => {println!("Memento");},
                            "moveRequest" => {println!("Move Request");},
                            "result" => {println!("Result");},
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

}

fn welcome_message(e: &BytesStart, data: &Mutex<GameData>) {
    let team = String::from_utf8(e.try_get_attribute("color").unwrap().unwrap().value.to_vec()).unwrap();
    println!("Received welcome message");
    println!("Team: {}", &team);
    data.lock().unwrap().set_team(&team);
}