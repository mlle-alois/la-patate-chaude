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
   /* let complexity = 1;
    let message = "Our red baker buys our lazy tree.";
    let hashCode = generate_hash(complexity,message);*/
    //println!("test : {:?}", create_seed(1,255));
    //let mut result:MD5HashCashOutput = MD5HashCashOutput { seed: 0, hashcode: "".to_string() };
    //let mut result:MD5HashCashInput = MD5HashCashInput { seed: 0, hashcode: "".to_string() };
    /*
    {"Challenge":{"MD5HashCash":{"complexity":1,"message":"Our red baker buys our lazy tree."}
    */
    let challengeResultMessage = Message::ChallengeResult(ChallengeResult {
        answer: MD5HashCash(generate_hash(1,"Our red baker buys our lazy tree.")),
        next_target: "test".to_string(),
    });
    let endOfGameString = {"EndOfGame\":{\"leader_board\":[{\"name\":\"jDA5uVaf4w\",\"stream_id\":\"127.0.0.1:43246\",\"score\":0,\"steps\":0,\"is_active\":false,\"total_used_time\":0.0},{\"name\":\"KMPMXW0D3K\",\"stream_id\":\"127.0.0.1:43248\",\"score\":0,\"steps\":0,\"is_active\":true,\"total_used_time\":0.0}]}}";
    let endOfGameMessage = serde_json::from_str(&endOfGameString).unwrap();
        println!("{:?}", endOfGameMessage);
    /*let hash = generate_hash(1,"Our red baker buys our lazy tree.");
    println!("{:?}", hash);*/
    let hash = generate_hash(9,"hello");
    println!("{:?}", hash);

    let is_valid = is_hashcode_valid(hash.hashcode, 9);
    println!("{:?}", is_valid);

    let digest = md5::compute(b"Our red baker buys our lazy tree.");
    assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");

    let challengeType = get_type(challengeResultMessage);
    println!("{:?}", challengeType);

    /*let obj = json!({"PublicLeaderBoard": Array([Object({"is_active": Bool(true), "name": String("2xjxkuzw6K"), "score": Number(0), "steps": Number(0), "stream_id": String("127.0.0.1:55990"), "total_used_time": Number(0.0)})])});
    println!("publicLeaderBoardjson : {:?}", publicLeaderBoardjson["PublicLeaderBoard"][0]);*/
}
fn generate_hash(complexity :u32,message: &str) -> MD5HashCashOutput{
    let mut verif = false;
    let mut index = 0;
    let mut hash:String;
    let mut seed:String;
    let mut result:MD5HashCashOutput = MD5HashCashOutput { seed: 0, hashcode: "".to_string() };
    loop  {
        seed = create_seed(complexity,index);
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
            result.hashcode=format!("{:x}", hashcode);
            break;
        }
        index=index+1;
    }
    result
}
fn create_seed(complexity :u32,val: u32)->String{
    let elem = format!("{:x}", val);
    if(elem.len() + complexity as usize > 16){
      panic!("Failed seed creation!");
    }
    let hexa= format!("{:01$x}", val, 16);
   // println!("{}", elem);
    hexa
}
fn convert_to_binary_from_hex(hex: String) -> String {
    let to_binary = hex[2 ..]
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
    let mut index = 0;
   for index in 0..complexity {
        if(val_in_binary.chars().nth(index as usize).unwrap() != '0'){
            verif = false;
        }
    }
    // If the number of 0s is > complexity, the hashcode is valid.
    println!("{:?}",verif);
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