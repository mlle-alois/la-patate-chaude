extern crate core;

use std::borrow::Borrow;
use std::fmt::format;
use std::io::{Read, Write};
use std::mem::size_of;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::vec;
//use crate::ChallengeAnswer::ChallengeAnswer::ChallengeName;
use serde::{Deserialize, Serialize};
use serde_json;
use shared::Models::{ChallengeAnswer, ChallengeValue};
use shared::Models::ChallengeOutput::ChallengeOutput;
use shared::Models::ChallengeResult::ChallengeResult;
use shared::Models::message::Message;
use shared::Models::message::Message::{ PublicLeaderBoard,Subscribe, Welcome};
use shared::Models::subscribe::subscribe;
use shared::Models::welcome::welcome;
use rand::Rng;
use shared::Models::ChallengeAnswer::ChallengeAnswer::MD5HashCash;
use shared::Models::PublicPlayer::PublicPlayer;

fn main() {
    //let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
    //assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
    // create multiple clients with TcpStream to connect to the server "localhost:7878"
/*
    let randomPlayerName = generate_random_string(10);
    let subscribe = Message::Subscribe(subscribe { name: randomPlayerName.parse().unwrap() });
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
            // Subcription player 1
            let serializeSubscribe = serializeMessage(&subscribe);
            writeMessage(&tcpStream, &serializeSubscribe);
            // SubscribeResult player 1
            let SubscribeResultLength = messageLength(&tcpStream);
            let SubscribeResult = readMessage(&tcpStream, SubscribeResultLength);
            println!("{:?}", SubscribeResult);


*/

    let playerName = generate_random_string(10);
    let mut tcpStream1 = connect_and_subscribe_player(playerName.clone());
            /** Round **/


    loop {
        // PublicLeaderBoard
        let publicLeaderBoardLenght = messageLength(&tcpStream1);
        let mut publicLeaderBoard = readMessage(&tcpStream1, publicLeaderBoardLenght);
        println!("{:?}", publicLeaderBoard);


        /*let other_players = get_other_players_name(&publicLeaderBoard,&playerName);
        println!("{:?}", other_players);
        let target = pick_random_player_name(&other_players);*/


        // Challenge
        let challengeLenght = messageLength(&tcpStream1);
        let challenge = readMessage(&tcpStream1, challengeLenght);
        println!("{:?}yo", challenge);

    // ChallengeResult
    let challengeResultMessage = Message::ChallengeResult(ChallengeResult {
        answer: MD5HashCash(ChallengeOutput {
            seed: 12345678,
            hashcode: "abcdefghijklmnopqrstuvwxyz".to_string(),
        }),
        next_target: "name".to_string(),

    });
    let serializeCR = serializeMessage(&challengeResultMessage);
    writeMessage(&tcpStream1, &serializeCR);

        // RoundSummary
        let roundSummaryLenght = messageLength(&tcpStream1);
        let roundSummary = readMessage(&tcpStream1, roundSummaryLenght);
        println!("{:?}", roundSummary);
    }
            /*
            // EndOfGame
            let endOfGameLenght = messageLength(&tcpStream);
            let endOfGame = readMessage(&tcpStream, endOfGameLenght);
            println!("{:?}", endOfGame);*/

       // Err(err) => panic!("Cannot connect : {err}")
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
                let data = serde_json::from_str::<Message>(&*x);
                data.unwrap()
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    }

fn get_other_players_name(publicLeaderBoard : &Message, playerToExclude: &String) -> Vec<String> {
    let mut other_players: Vec<String> = vec![];
    let publicLeaderBoard = publicLeaderBoard.clone();
    match publicLeaderBoard {
        Message::PublicLeaderBoard(publicLeaderBoard) => {
            for player in publicLeaderBoard {
                if player.name.ne(playerToExclude) {
                    other_players.push(player.name.clone());
                }
            }
        }
        _ => {
            panic!("Not a PublicLeaderBoard");
        }

    }
    other_players

    //result.players.filter(|x| x.name.ne( playerToExclude)).collect()
    //publicLeaderBoard::<PublicLeaderBoard>().players.iter().filter(|player| player.name != playerToExclude).map(|player| player.clone()).collect()

}
fn pick_random_player_name(player_names: &Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, player_names.len());
    player_names[index].clone()
}


    // generate random string of length 10
    fn generate_random_string(len: usize) -> String {
        let mut rng = rand::thread_rng();
        let random_string: String = (0..len).map(|_| rng.sample(rand::distributions::Alphanumeric)).collect();
        random_string
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
    fn determine_complexity(hashcode: &str) -> u32 {
        let mut count = 0;
        for c in hashcode.chars() {
            if c == '0' {
                count += 1;
            }
        }
        count
    }
    fn is_hashcode_valid(hashcode: &str, complexity: u32) -> bool {
        determine_complexity(hashcode) > complexity
    }
    fn is_hashcode_valid_with_seed(hashcode: &str, complexity: u32, seed: u32) -> bool {
        determine_complexity(hashcode) > complexity
    }
    fn is_hashcode_valid_with_seed_and_message(hashcode: &str, complexity: u32, seed: u32, message: &str) -> bool {
        determine_complexity(hashcode) > complexity
    }

    fn format_seed_and_message(seed: u32, message: &str) -> String {
        let format = format!("{:x}", seed);
        let hashcode = md5::compute(format.as_bytes());
        let hashcode = format!("{:x}", hashcode);
        format!("{}{}", hashcode, message)
    }

    fn connect_and_subscribe_player(name:String) -> TcpStream {
        let subscribe = Message::Subscribe(subscribe { name: name.parse().unwrap() });
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
                // Subcription player 1
                let serializeSubscribe = serializeMessage(&subscribe);
                writeMessage(&tcpStream, &serializeSubscribe);
                // SubscribeResult player 1
                let SubscribeResultLength = messageLength(&tcpStream);
                let SubscribeResult = readMessage(&tcpStream, SubscribeResultLength);
                println!("{:?}", SubscribeResult);
                tcpStream
            }

            Err(err) => panic!("Cannot connect : {err}")
        }
    }




