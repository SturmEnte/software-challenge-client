mod utils;
mod parse_message;
mod game_data;

use std::sync::Mutex;
use std::net::TcpStream;
use std::io::{Write, Read, Cursor};

use game_data::GameData;
use utils::get_room_id::get_room_id;
use parse_message::parse_message;

const SERVER_ADRESS: &str = "127.0.0.1:13050";

fn main() {
    let game_data: Mutex<GameData> = Mutex::new(GameData::new());

    let mut stream = TcpStream::connect(SERVER_ADRESS).unwrap();

    // Send join message
    stream.write("<protocol><join/>".as_bytes()).unwrap();

    let mut global_buffer: Cursor<[u8; 5000]> = Cursor::new([0; 5000]);
    let mut global_n: usize = 0usize;
    let mut msg = 0;

    // Create folder for msgs if it doesnt exist
    let _r = std::fs::create_dir("msg");

    loop {
        let mut buffer = [0; 5000];

        let n = stream.read(&mut buffer[..]).unwrap();

        if buffer.starts_with(b"<protocol>") {
            println!("Joined room");
            println!("Room id: {}", get_room_id(&buffer));
        } else if buffer[n-7..n] == "</room>".as_bytes().to_owned() { // returns true, if the data in the buffer ends with </room>
            global_buffer.write(&buffer[..n]).unwrap();
            global_n += n;

            //let g_buff_in = global_buffer.into_inner();
            // println!("Message: \n{}", from_utf8(&g_buff_in[..global_n]).unwrap());
            // let mut file = std::fs::File::create(format!("msg/msg{msg}.xml")).unwrap();
            // file.write(&g_buff_in[..global_n]).unwrap();
            // msg += 1;
            // parse_room_msg(g_buff_in);

            parse_message(global_buffer.into_inner(), global_n, &game_data);

            global_buffer = Cursor::new([0; 5000]);
            global_n = 0usize;
        } else {
            // Add buffer data to the global buffer and add n to the global n
            global_buffer.write(&buffer[..n]).unwrap();
            global_n += n;
        }
    }
}
