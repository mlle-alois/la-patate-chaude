extern crate core;

use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt::format;
use std::io::{Read, Write};
use std::mem::size_of;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::vec;

use crate::ChallengeAnswer::ChallengeAnswer::ChallengeName;

fn main() {
    //let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
    //assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
    let message = format_seed_and_message(9,"hello");
    println!("test : {:?}", message);
}

fn format_seed_and_message(seed: u32, message: &str) -> String {
    let format = format!("{:x}", seed);
    let hashcode = md5::compute(format.as_bytes());
    let hashcode = format!("{:x}", hashcode);
    format!("{}{}", hashcode, message)
}


