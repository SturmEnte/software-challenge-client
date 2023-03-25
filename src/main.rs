mod utils;
mod parse_message;
mod parse_memento;
mod game_data;
mod board;
mod compute;
mod game_move;

use std::sync::Mutex;
use std::net::TcpStream;
use std::io::{Write, Read, Cursor};
use std::fs::File;

use game_data::GameData;
use game_move::Move;
use utils::get_room_id::get_room_id;
use utils::get_cmd_args::get_join_info;
use parse_message::parse_message;
use parse_memento::parse_memento_from_str;
use compute::compute_move;

const COMPUTE_TEST: bool = false;
const VERSION: u8 = 2;

fn main() {
    println!("Version: {}", VERSION);
    println!("Compute test: {}", COMPUTE_TEST);
    let game_data: Mutex<GameData> = Mutex::new(GameData::new());

    let join_info: (String, String) = get_join_info();
    let server_address: &str = join_info.0.as_str();
    let join_msg: &str = join_info.1.as_str();
     
    let mut global_buffer: Cursor<[u8; 5000]> = Cursor::new([0; 5000]);
    let mut global_n: usize = 0usize;
    let mut _msg: i32 = 0;

    if COMPUTE_TEST {
        let mut file = File::open("mementos/memento.xml").expect("File \"mementos/memento.xml\" could not be found");
        
        let mut memento = String::new();
        file.read_to_string(&mut memento).unwrap();
        println!("{}", memento);
        
        parse_memento_from_str(&memento, &game_data);

        compute_move(&game_data);

    } else {
        let mut stream = TcpStream::connect(server_address).unwrap();

        // Send join message
        stream.write(join_msg.as_bytes()).unwrap();

        loop {
            let mut buffer: [u8; 5000] = [0; 5000];

            let n: usize = stream.read(&mut buffer[..]).unwrap();
            
            if buffer.starts_with(b"<protocol>") {
                println!("Joined room");
                game_data.lock().unwrap().room_id = get_room_id(&buffer);
                println!("Room id: {}", game_data.lock().unwrap().room_id);

            } else if buffer[n-7..n] == "</room>".as_bytes().to_owned() { // returns true, if the data in the buffer ends with </room>
                global_buffer.write(&buffer[..n]).unwrap();
                global_n += n;

                let game_end: bool = parse_message(global_buffer.into_inner(), global_n, &game_data, &mut Some(&mut stream));

                if game_end {
                    break;
                }

                global_buffer = Cursor::new([0; 5000]);
                global_n = 0usize;
            } else {
                // Add buffer data to the global buffer and add n to the global n
                global_buffer.write(&buffer[..n]).unwrap();
                global_n += n;
            }
        }
    }
}
