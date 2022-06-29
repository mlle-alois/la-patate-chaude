extern crate core;

use std::fmt::format;
use std::io::{Read, Write};
use std::mem::size_of;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::vec;

use serde::{Deserialize, Serialize};
use serde_json;

use shared::Models::message::Message;
use shared::Models::message::Message::Welcome;
use shared::Models::welcome::welcome;

fn main() {
    //let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
    //assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
    // create multiple clients with TcpStream to connect to the server "localhost:7878"

    //let h = Message::Welcome(Welcome { version: 2 });
    let helloMessage = Message::Hello;

//Serialize message
    let serializeHM = serializeMessage(&helloMessage);

    let mut tcpStream = TcpStream::connect("localhost:7878");
    match tcpStream {
        Ok(mut tcpStream) => {
            writeMessage(&tcpStream, &serializeHM);
            let helloMessageLenght =messageLength(&tcpStream);
            readMessage(&tcpStream,helloMessageLenght);
        }
        Err(err) => panic!("Cannot connect : {err}")
    }

    fn serializeMessage(msg: &Message) -> String {
        let serialized = serde_json::to_string(&msg);
        match serialized {
            Ok(mut str) => {
                println!("ok:{str}");
                str
            }
            Err(err) => {
                panic!("failed");
            }
        }
    }
    fn writeMessage(mut stream: &TcpStream, content: &String) {
        println!("{}", content);
        let mut messageLenght: u32 = content.len() as u32;
        let prefix = messageLenght.to_be_bytes();
        println!("{:?}", prefix);

        let message = content.as_bytes();

        stream.write_all(&prefix);
        stream.write_all(&message);
    }

    fn messageLength(mut stream: &TcpStream) -> usize{
        let mut bufferLenght = vec![0u8; 4];
        let res = stream.read_exact(&mut bufferLenght).unwrap();
        println!("{:?} res", &bufferLenght[3]);
         *&bufferLenght[3] as usize;
    }
    fn readMessage(mut stream: &TcpStream,messageLenght:usize) {
        let mut buffer = vec![0u8; messageLenght];
        let res = stream.read_exact(&mut buffer);
        match res {
            Ok(_) => {
                println!("{:?}", buffer);
                let x = String::from_utf8_lossy(&buffer);
                println!("{:?}dd", x);
                //let welcome = serde_json::from_slice::<welcome>(&buffer);
                let data = serde_json::from_str::<Message>(&*x);
                println!("{:?}", data.unwrap());
                data.unwrap();
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
}



