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

fn main() {
    println!("Hello, world!");
    let complexity = 4;
    let message = "hello";
    let hashCode = generate_hash(complexity,message);
    println!("test : {:?}", create_seed(9,255));
}
fn generate_hash(complexity :u32,message: &str){
    let mut verif = false;
    let mut index = 1;
    let mut seed:String;
    loop  {
        seed = create_seed(complexity,index);
        let elem = format!("{}{}", seed, message);
       // println!("elem : {:?}", elem);
        let hashcode = md5::compute(elem);
        println!("seed : {:?} hashcode : {:?} ", seed,hashcode);
        let str: String =format!("{:x}", hashcode);
       // println!("hashcode : {:?}", str);
        verif=is_hashcode_valid(str,complexity);
        index=index+1;
        if(verif){
            println!("hashcode : {:?}", hashcode);
            break;
        }
    }
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