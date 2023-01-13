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

use game_data::GameData;
use game_move::Move;
use utils::get_room_id::get_room_id;
use utils::get_cmd_args::get_join_info;
use parse_message::parse_message;

fn main() {
    let game_data: Mutex<GameData> = Mutex::new(GameData::new());

    let join_info: (String, String) = get_join_info();
    let server_address: &str = join_info.0.as_str();
    let join_msg: &str = join_info.1.as_str();
     
    let mut stream = TcpStream::connect(server_address).unwrap();

    // Send join message
    stream.write(join_msg.as_bytes()).unwrap();

    let mut global_buffer: Cursor<[u8; 5000]> = Cursor::new([0; 5000]);
    let mut global_n: usize = 0usize;
    let mut _msg = 0;

    // Create folder for msgs if it doesnt exist
    let _r = std::fs::create_dir("msg");

    loop {
        let mut buffer = [0; 5000];

        let n = stream.read(&mut buffer[..]).unwrap();

        if buffer.starts_with(b"<protocol>") {
            println!("Joined room");
            game_data.lock().unwrap().room_id = get_room_id(&buffer);
            println!("Room id: {}", game_data.lock().unwrap().room_id);

        } else if buffer[n-7..n] == "</room>".as_bytes().to_owned() { // returns true, if the data in the buffer ends with </room>
            global_buffer.write(&buffer[..n]).unwrap();
            global_n += n;

            // let g_buff_in = global_buffer.into_inner();
            // println!("Message: \n{}", std::str::from_utf8(&g_buff_in[..global_n]).unwrap());
            // let mut file = std::fs::File::create(format!("msg/msg{msg}.xml")).unwrap();
            // file.write(&g_buff_in[..global_n]).unwrap();
            // msg += 1;

            let game_end = parse_message(global_buffer.into_inner(), global_n, &game_data, &mut stream);

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
