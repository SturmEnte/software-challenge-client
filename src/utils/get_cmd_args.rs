use std::env;

pub fn get_join_info() -> (String, String) {
    let args: Vec<String> = env::args().collect();
    let mut port: &String = &String::from("13050");
    let mut host: &String = &String::from("127.0.0.1");
    let mut game: &String = &String::from("");
    let mut join_msg: String = String::from("<protocol><join/>");
    for (i, arg) in args.iter().enumerate() {
        if arg == "--port" || arg == "-p" {
            port = &args[i+1];
        }
        else if arg == "--host" || arg == "-h" {
            host = &args[i+1];
        }
        if arg == "--reservation" || arg == "-r" {
            game = &args[i+1];
        }
    }
    let server_address: String = format!("{}:{}", host, port);
    if game != "" {
        join_msg = format!("<protocol><joinPrepared reservationCode=\"{}\" />", game);
    }
    println!("Server Address: {}", server_address);
    println!("Reservation Code: {}", game);
    println!("Join Message: {}", join_msg);
    
    return (server_address, join_msg);
}
