extern crate core;

use std::fmt::format;
use std::io::{Read, Write};
use std::mem::size_of;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::vec;
use crate::ChallengeAnswer::ChallengeAnswer::ChallengeName;
use serde::{Deserialize, Serialize};
use serde_json;
use shared::Models::{ChallengeAnswer, ChallengeValue};
use shared::Models::ChallengeOutput::ChallengeOutput;
use shared::Models::ChallengeResult::ChallengeResult;

use shared::Models::message::Message;
use shared::Models::message::Message::{Subscribe, Welcome};
use shared::Models::subscribe::subscribe;
use shared::Models::welcome::welcome;

fn main() {
    //let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
    //assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
    // create multiple clients with TcpStream to connect to the server "localhost:7878"
    let subscribe = Message::Subscribe(subscribe{name: "yolo".parse().unwrap() });
    //let h = Message::Welcome(Welcome { version: 2 });


    let mut tcpStream = TcpStream::connect("localhost:7878");
    match tcpStream {
        Ok(mut tcpStream) => {
            // Hello
            let helloMessage = Message::Hello;
            let serializeHM = serializeMessage(&helloMessage);
            writeMessage(&tcpStream, &serializeHM);
            // Welcome
            let welcomeMessageLenght = messageLength(&tcpStream);
            let welcomeMessage = readMessage(&tcpStream, welcomeMessageLenght);
            println!("{:?}", welcomeMessage);
            // Subcription
            let serializeSubscribe = serializeMessage(&subscribe);
            writeMessage(&tcpStream, &serializeSubscribe);
            // SubscribeResult
            let SubscribeResultLength= messageLength(&tcpStream);
            let SubscribeResult = readMessage(&tcpStream, SubscribeResultLength);
            println!("{:?}", SubscribeResult);

            /** Round **/
            // PublicLeaderBoard
            let publicLeaderBoardLenght = messageLength(&tcpStream);
            let publicLeaderBoard = readMessage(&tcpStream, publicLeaderBoardLenght);
            println!("{:?}", publicLeaderBoard);
            // Challenge
            let challengeLenght = messageLength(&tcpStream);
            let challenge = readMessage(&tcpStream, challengeLenght);
            println!("{:?}", challenge);
            // ChallengeResult
            let challengeResultMessage = Message::ChallengeResult(ChallengeResult {
                name: ChallengeName(ChallengeOutput {
                    seed: 12345678,
                    hashcode: "abcdefghijklmnopqrstuvwxyz".to_string(),
                }),
                next_target: "yolo".to_string(),

            });
            let serializeCR = serializeMessage(&challengeResultMessage);
            writeMessage(&tcpStream, &serializeCR);

            // RoundSummary
            let roundSummaryLenght = messageLength(&tcpStream);
            let roundSummary = readMessage(&tcpStream, roundSummaryLenght);
            println!("{:?}", roundSummary);

            // EndOfGame
            let endOfGameLenght = messageLength(&tcpStream);
            let endOfGame = readMessage(&tcpStream, endOfGameLenght);
            println!("{:?}", endOfGame);
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

    fn messageLength(mut stream: &TcpStream) -> usize {
        let mut bufferLenght = [0u8; 4];
        let res = stream.read_exact(&mut bufferLenght).unwrap();
        let value = u32::from_be_bytes(bufferLenght);
        println!("{:?} res", &value);
        *&value as usize
    }
    fn readMessage(mut stream: &TcpStream, messageLenght: usize) -> Message {
        let mut buffer = vec![0u8; messageLenght];
        let res = stream.read_exact(&mut buffer);
        match res {
            Ok(_) => {
                println!("{:?}", buffer);
                let x = String::from_utf8_lossy(&buffer);
                println!("{:?}dd", x);
                //let welcome = serde_json::from_slice::<welcome>(&buffer);
                let data = serde_json::from_str::<Message>(&*x);
                data.unwrap()
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    }
    // stays string
    // seed :decimal += 1;
    // format : hexadecimal = format!("{:x}", seed);
    // seed + message
    // hashcode = md5::compute(format.as_bytes());
    // hashcode: hexadecimal = format!("{:x}", hashcode);
    // format hashcode to binary
    // Determine the number of 0s in the binary representation of the hashcode.
    // If the number of 0s is > complexity, the hashcode is valid.

}


