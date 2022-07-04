extern crate core;

use std::char::from_digit;
use std::fmt::format;
use std::io::{Read, Write};
use std::mem::size_of;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::vec;
use rand::Rng;

use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
use shared::Models::Challenge::Challenge;
use shared::Models::ChallengeAnswer::ChallengeAnswer::MD5HashCash;
use shared::Models::ChallengeResult::ChallengeResult;
use shared::Models::MD5HashCashInput::MD5HashCashInput;
use shared::Models::MD5HashCashOutput::MD5HashCashOutput;
use shared::Models::message::Message;

fn main() {
    println!("Hello, world!");
    let complexity = 2;
    let message = "A chatty the Bernardo'sory carries my smart spoon.";
    let hashCode = generate_hash(complexity,message);
    println!("hashcode : {:?} ", hashCode);


}

fn generate_hash(complexity :u32,message: &str) -> MD5HashCashOutput{
    let mut verif = false;
    let mut index = 0;
    let mut hash:String;
    let mut seed:String;
    let mut result:MD5HashCashOutput = MD5HashCashOutput { seed: 0, hashcode: "".to_string() };
    loop  {
        seed = create_seed(index);
        println!("seed : {:?} ", seed);
        let elem = format!("{}{}", seed, message);
        // println!("elem : {:?}", elem);
        let hashcode = md5::compute(elem);
        //   println!("seed : {:?} hashcode : {:?} ", seed,hashcode);
        hash =format!("{:x}", hashcode);
        // println!("hashcode : {:?}", str);
        verif=is_hashcode_valid(hash,complexity);
        if(verif){
            //  println!("hashcode : {:?}", hashcode);
            result.seed= index as u64;
            result.hashcode=format!("{:x}", hashcode).to_uppercase();
            break;
        }
        index=index+1;
    }
    result
}

fn create_seed(val: u32)->String{
    let hexa= format!("{:01$x}", val, 16);
    // println!("{}", elem);
    hexa.to_uppercase()
}
fn convert_to_binary_from_hex(hex: String) -> String {
    let to_binary = hex
        .chars()
        .map(|c| to_binary(c))
        .collect();
    to_binary
}

fn get_type(dataType: Message) -> String {
    match dataType {
        Message::Challenge(_) => {
            "Challenge".to_string()
        }
        Message::ChallengeResult(_) => {
            "ChallengeResult".to_string()
        }
        _ => {"".to_string()}
    }
}


fn is_hashcode_valid(hashcode: String, complexity: u32) -> bool {
   let mut val_in_binary = convert_to_binary_from_hex(hashcode.to_uppercase());
    println!("hashcode : {:?} val_in_binary : {:?}", hashcode,val_in_binary);
    let mut verif = true;
   for index in 0..complexity {
        if(val_in_binary.chars().nth(index as usize).unwrap() != '0'){
            verif = false;
        }
    }
    verif
}

fn to_binary(c: char) -> String {
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
        _  => "",
    };
    b.to_string()
}