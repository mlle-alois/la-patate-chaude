use std::io::Write;
use std::net::TcpStream;

fn main() {
    println!("Hello, world!");
    //let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
    //assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
    // create multiple clients with TcpStream to connect to the server "localhost:7878"
    let mut stream = TcpStream::connect("localhost:7878");
    match stream {
        Ok(mut stream) => {
            let message= "Hello".as_bytes();
            let response = stream.write_all(&message);
        }
        Err(err) => panic!("Cannot connect : {err}"),
    }

}
