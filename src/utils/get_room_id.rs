use quick_xml::Reader;
use quick_xml::events::Event;

pub fn get_room_id(xml: &[u8]) -> String {
    let mut reader: Reader<&[u8]> = Reader::from_bytes(xml);
    reader.trim_text(true);
    reader.expand_empty_elements(true);

    let mut buf: Vec<u8> = Vec::new();
    
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
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), 
        }
        buf.clear();
    }

    panic!("No room id found")
}