use std::net::TcpStream;
use std::io::{Write, Read, Cursor};
use std::str::from_utf8;
use quick_xml::Reader;
use quick_xml::events::Event;

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

fn get_room_id(xml: &[u8]) -> String {
    let mut reader = Reader::from_bytes(xml);
    reader.trim_text(true);
    reader.expand_empty_elements(true);

    let mut txt = Vec::new();
    let mut buf = Vec::new();
    
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"joined" => {
                        return String::from_utf8(e.try_get_attribute("roomId").unwrap().unwrap().value.to_vec()).unwrap();
                    },
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), 
        }
        buf.clear();
    }

    panic!("No room id found")
}