mod utils;

use std::net::TcpStream;
use std::io::{Write, Read, Cursor};
use std::str::from_utf8;

use utils::get_room_id::*;

const SERVER_ADRESS: &str = "127.0.0.1:13050";

fn main() {
    let mut stream = TcpStream::connect(SERVER_ADRESS).unwrap();

    stream.write("<protocol><join/>".as_bytes()).unwrap();

    let mut global_buffer: Cursor<[u8; 100000]> = Cursor::new([0; 100000]);
    let mut global_n: usize = 0usize;
    let mut msg = 0;

    let _r = std::fs::create_dir("msg");

    loop {
        let mut buffer = [0; 100000];

        let n = stream.read(&mut buffer[..]).unwrap();

        if buffer.starts_with(b"<protocol>") {
            println!("Joined room");
            println!("Room id: {}", get_room_id(&buffer));
        } else if buffer[n-7..n] == "</room>".as_bytes().to_owned() {
            global_buffer.write(&buffer[..n]).unwrap();
            global_n += n;

            let g_buff_in = global_buffer.into_inner();
            println!("Message: \n{}", from_utf8(&g_buff_in[..global_n]).unwrap());
            let mut file = std::fs::File::create(format!("msg/msg{msg}.xml")).unwrap();
            file.write(&g_buff_in[..global_n]).unwrap();
            msg += 1;

            global_buffer = Cursor::new([0; 100000]);
            global_n = 0usize;
        } else {
            global_buffer.write(&buffer[..n]).unwrap();
            global_n += n;
        }
    }
}
