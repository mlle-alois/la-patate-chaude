extern crate core;
pub mod HashCash;
use std::borrow::Borrow;
use std::fmt::format;
use std::io::{Read, Write};
use std::mem::size_of;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::vec;
use serde::{Deserialize, Serialize};
use serde_json;
use shared::Models::{ChallengeAnswer, ChallengeValue, message};
use shared::Models::ChallengeResult::ChallengeResult;
use shared::Models::message::Message;
use shared::Models::message::Message::{PublicLeaderBoard, Subscribe, Welcome};
use shared::Models::subscribe::subscribe;
use shared::Models::welcome::welcome;
use rand::Rng;
use serde_json::json;
use shared::Models::Challenge::Challenge;

use shared::Models::ChallengeAnswer::ChallengeAnswer::MD5HashCash;
use shared::Models::MD5HashCashOutput::MD5HashCashOutput;
use shared::Models::PublicPlayer::PublicPlayer;
use crate::HashCash::{connect_and_subscribe_player, generate_random_string,
                      get_other_players_name, get_type, messageLength,
                      pick_random_player_name, processChallenge, readMessage
};

fn main() {
    let playerName = generate_random_string(10);
    let mut tcpStream1 = connect_and_subscribe_player(playerName.clone());
    /** Round **/


    loop {
        // PublicLeaderBoard
        let publicLeaderBoardLenght = messageLength(&tcpStream1);
        let mut publicLeaderBoard = readMessage(&tcpStream1, publicLeaderBoardLenght);
        println!("{:?}", publicLeaderBoard);

        let endLoopType =get_type(&publicLeaderBoard);
        if endLoopType == "EndOfGame" || endLoopType == "Excluded" {
            break;
        }


        let other_players = get_other_players_name(&publicLeaderBoard, &playerName);
        println!("{:?}", other_players);
        let target = pick_random_player_name(&other_players);


        // Challenge OR RoundSummary
        let messageLenght = messageLength(&tcpStream1);
        let mut message = readMessage(&tcpStream1, messageLenght);
        println!("{:?}", message);

        let mut messageType = get_type(&message);

        while messageType == "Challenge" {
            processChallenge(&target, &mut tcpStream1, &message);

            // RoundSummary
            let roundSummaryLenght = messageLength(&tcpStream1);
            let roundSummary = readMessage(&tcpStream1, roundSummaryLenght);
            println!("{:?}", roundSummary);

            messageType = get_type(&roundSummary);
            message = roundSummary;
        }
    }
}




