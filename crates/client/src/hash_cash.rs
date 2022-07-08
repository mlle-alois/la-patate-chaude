extern crate core;

use std::borrow::Borrow;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::vec;
use serde_json;
use shared::models::challenge_result::ChallengeResult;
use shared::models::message::Message;
use shared::models::subscribe::Subscribe;
use rand::Rng;
use serde_json::json;
use shared::models::challenge::Challenge;
use shared::models::challenge_answer::ChallengeAnswer::MD5HashCash;
use shared::models::md5hash_cash_output::MD5HashCashOutput;

pub fn process_challenge(player_name: &String, tcp_stream1: &mut TcpStream, message: &Message) {
    let challenge = message;

    match challenge {
        Message::Challenge(challenge) => {
            println!("md5: {:?}yo", challenge);
            let md5 = &challenge;
            match md5 {
                Challenge::MD5HashCash(md5) => {
                    //println!("md5: {:?}yo", md5.complexity);
                    //println!("md5: {:?}yo", md5.message);
                    // ChallengeResult
                    let challenge_result_message = Message::ChallengeResult(ChallengeResult {
                        answer: MD5HashCash(generate_hash(md5.complexity, &md5.message.clone())),
                        next_target: player_name.to_string(),

                    });
                    let serialize_cr = serialize_message(&challenge_result_message);
                    write_message(&tcp_stream1, &serialize_cr);
                }

            }
        }
        _ => {
            panic!("Not a Challenge");
        }
    }
}

pub fn get_type(data_type: &Message) -> String {
    match data_type {
        Message::Challenge(_) => {
            "Challenge".to_string()
        }
        Message::ChallengeResult(_) => {
            "ChallengeResult".to_string()
        }
        Message::Welcome(_) => {
            "Welcome".to_string()
        }
        Message::Hello => {
            "Hello".to_string()
        }
        Message::Subscribe(_) => {
            "Subscribe".to_string()
        }
        Message::SubscribeResult(_) => {
            "SubscribeResult".to_string()
        }
        Message::EndOfGame(_) => {
            "EndOfGame".to_string()
        }
        Message::Excluded(_) => {
            "Excluded".to_string()
        }
        Message::RoundSummary(_) => {
            "RoundSummary".to_string()
        }
        _ => { "".to_string() }
    }
}

pub(crate) fn serialize_message(msg: &Message) -> String {
    let serialized = serde_json::to_string(&msg);
    match serialized {
        Ok(str) => {
            println!("ok:{str}");
            str
        }
        Err(_) => {
            panic!("failed");
        }
    }
}

pub fn write_message(mut stream: &TcpStream, content: &String) {
    println!("{}", content);
    let  message_length: u32 = content.len() as u32;
    let prefix = message_length.to_be_bytes();
    println!("{:?}", prefix);

    let message = content.as_bytes();

    stream.write_all(&prefix);
    stream.write_all(&message);
}

pub fn message_length(mut stream: &TcpStream) -> usize {
    let mut buffer_length = [0u8; 4];
    stream.read(&mut buffer_length).unwrap();
    let value = u32::from_be_bytes(buffer_length);
    println!("{:?} res", &value);
    *&value as usize
}

pub fn read_message(mut stream: &TcpStream, message_lenght: usize) -> Message {
    if message_lenght == 0 {
        return Message::Excluded("".to_string());
    }
    let mut buffer = vec![0u8; message_lenght];
    let res = stream.read(&mut buffer);
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

pub fn get_other_players_name(message: &Message, player_to_exclude: &String) -> Vec<String> {
    let mut other_players: Vec<String> = vec![];
    let public_leader_board_tmp = message.clone();
    match public_leader_board_tmp {
        Message::PublicLeaderBoard(players) => {
            for player in players {
                if player.name.ne(player_to_exclude) {
                    other_players.push(player.name.clone());
                }
            }
        }
        _ => {
            panic!("Not a PublicLeaderBoard");
        }
    }
    other_players
}

pub fn pick_random_player_name(player_names: &Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, player_names.len());
    player_names[index].clone()
}

pub fn generate_random_string(len: usize) -> String {
    let mut rng = rand::thread_rng();
    let random_string: String = (0..len).map(|_| rng.sample(rand::distributions::Alphanumeric)).collect();
    random_string
}

pub fn generate_hash(complexity: u32, message: &str) -> MD5HashCashOutput {
    let mut verif = false;
    let mut index = 0;
    let mut hash: String;
    let mut seed: String;
    let mut result: MD5HashCashOutput = MD5HashCashOutput { seed: 0, hashcode: "".to_string() };
    loop {
        seed = create_seed(index);
        //println!("seed : {:?} ", seed);
        let elem = format!("{}{}", seed, message);
        // println!("elem : {:?}", elem);
        let hashcode = md5::compute(elem);
        //   println!("seed : {:?} hashcode : {:?} ", seed,hashcode);
        hash = format!("{:x}", hashcode);
        // println!("hashcode : {:?}", str);
        verif = is_hashcode_valid(hash, complexity);
        if verif {
            //  println!("hashcode : {:?}", hashcode);
            result.seed = index as u64;
            result.hashcode = format!("{:x}", hashcode).to_uppercase();
            break;
        }
        index = index + 1;
    }
    result
}

pub fn create_seed(val: u32) -> String {
    let hexa = format!("{:01$x}", val, 16);
    // println!("{}", elem);
    hexa.to_uppercase()
}

pub fn convert_to_binary_from_hex(hex: String) -> String {
    let to_binary = hex
        .chars()
        .map(|c| to_binary(c))
        .collect();
    to_binary
}

pub fn is_hashcode_valid(hashcode: String, complexity: u32) -> bool {
    let val_in_binary = convert_to_binary_from_hex(hashcode.to_uppercase());
    //println!("hashcode : {:?} val_in_binary : {:?}", hashcode, val_in_binary);
    let mut verif = true;
    for index in 0..complexity {
        if val_in_binary.chars().nth(index as usize).unwrap() != '0' {
            verif = false;
        }
    }
    verif
}

pub fn to_binary(c: char) -> String {
    let b = match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    };
    b.to_string()
}

pub fn connect_player(name: &String) -> TcpStream {

    let tcp_stream = TcpStream::connect("localhost:7878");
    match tcp_stream {
        Ok(tcp_stream_tmp) => {
            tcp_stream_tmp
        }
        Err(err) => panic!("Cannot connect : {err}")
    }
}

pub fn subscribe_player(name: &String, tcp_stream_tmp: &TcpStream) {
// Hello
    let subscribe = Message::Subscribe(Subscribe { name: name.parse().unwrap() });
    let hello_message = Message::Hello;
    let serialize_hm = serialize_message(&hello_message);
    write_message(&tcp_stream_tmp, &serialize_hm);
    // Welcome
    let welcome_message_lenght = message_length(&tcp_stream_tmp);
    let welcome_message = read_message(&tcp_stream_tmp, welcome_message_lenght);
    println!("{:?}", welcome_message);
    // Subcription player 1
    let serialize_subscribe = serialize_message(&subscribe);
    write_message(&tcp_stream_tmp, &serialize_subscribe);
    // subscribe_result player 1
    let subscribe_result_length = message_length(&tcp_stream_tmp);
    let subscribe_result = read_message(&tcp_stream_tmp, subscribe_result_length);
    println!("{:?}", subscribe_result);
}
