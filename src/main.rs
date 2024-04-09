use std::io::BufRead;

fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:9999").unwrap();
    for mut stream in listener.incoming().flatten() {
        let mut rdr = std::io::BufReader::new(&mut stream);
        let mut l = String::new();
        loop {
            rdr.read_line(&mut l).unwrap();
            print!("{l}");
        }
    }
}
