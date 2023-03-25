// PROBLEM!!!!!
// CLIENT EXECUTES MOVES FROM EMTPY SPACES OR OPPONENT SPACES INSTED OF FROM PLAYERS SOMETIEMES!!!!
// I THINK THEY ARE PARSED INCORRECTLY AND PUT IN THE BOARD INCORRECTLY!!

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
use std::vec;

use game_data::GameData;
use game_move::Move;
use utils::get_room_id::get_room_id;
use utils::get_cmd_args::get_join_info;
use parse_message::parse_message;
use parse_memento::parse_memento_from_str;
use compute::compute_move;

const REPLAY_MODE: bool = false;
const COMPUTE_TEST: bool = true;

fn main() {
    println!("Version: 2");
    println!("Replay mode: {}", REPLAY_MODE);
    let game_data: Mutex<GameData> = Mutex::new(GameData::new());

    let join_info: (String, String) = get_join_info();
    let server_address: &str = join_info.0.as_str();
    let join_msg: &str = join_info.1.as_str();
     
    let mut global_buffer: Cursor<[u8; 5000]> = Cursor::new([0; 5000]);
    let mut global_n: usize = 0usize;
    let mut _msg: i32 = 0;

    // // Create folder for msgs if it doesnt exist
    // let _r = std::fs::create_dir("msg");

    if REPLAY_MODE {

        let mut file = File::open("replays/replay.xml").expect("File \"replays/replay.xml\" could not be found");
        // let mut contents = String::new(); file.read_buf(buf)
        // file.read_to_string(&mut contents).expect("Unable to read the file");
        // let mut messages = contents.split("</room>");

        let mut replay = String::new();

        file.read_to_string(&mut replay).unwrap();

        println!("{}", replay);

        loop {
            let mut buffer: [u8; 5000] = [0; 5000];

            let n: usize = file.read(&mut buffer[..]).unwrap();

            //println!("{:?}",buffer);
            
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

                let game_end: bool = parse_message(global_buffer.into_inner(), global_n, &game_data, &mut None, &REPLAY_MODE);

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

    } else if COMPUTE_TEST {
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

                // let g_buff_in = global_buffer.into_inner();
                // println!("Message: \n{}", std::str::from_utf8(&g_buff_in[..global_n]).unwrap());
                // let mut file = std::fs::File::create(format!("msg/msg{msg}.xml")).unwrap();
                // file.write(&g_buff_in[..global_n]).unwrap();
                // msg += 1;

                let game_end: bool = parse_message(global_buffer.into_inner(), global_n, &game_data, &mut Some(&mut stream), &REPLAY_MODE);

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
