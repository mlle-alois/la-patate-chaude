extern crate core;
pub mod hash_cash;
use serde_json;
use crate::hash_cash::{connect_and_subscribe_player, generate_random_string,
                       get_other_players_name, get_type, message_length,
                       pick_random_player_name, process_challenge, read_message
};

fn main() {
    let player_name = generate_random_string(10);
    let mut tcp_stream1 = connect_and_subscribe_player(player_name.clone());
    /** Round **/


    loop {
        // PublicLeaderBoard
        let public_leader_board_length = message_length(&tcp_stream1);
        let  public_leader_board = read_message(&tcp_stream1, public_leader_board_length);
        println!("{:?}", public_leader_board);

        let end_loop_type =get_type(&public_leader_board);
        if end_loop_type == "EndOfGame" {
            break;
        }
        if end_loop_type == "Excluded" {
            println!("Player {:?} is excluded", player_name);
            break;
        }


        let other_players = get_other_players_name(&public_leader_board, &player_name);
        println!("{:?}", other_players);
        let target = pick_random_player_name(&other_players);


        // Challenge OR RoundSummary
        let message_lenght = message_length(&tcp_stream1);
        let mut message = read_message(&tcp_stream1, message_lenght);
        println!("{:?}", message);

        let mut message_type = get_type(&message);

        while message_type == "Challenge" {
            process_challenge(&target, &mut tcp_stream1, &message);

            // RoundSummary
            let round_summary_lenght = message_length(&tcp_stream1);
            let round_summary = read_message(&tcp_stream1, round_summary_lenght);
            println!("{:?}", round_summary);

            message_type = get_type(&round_summary);
            message = round_summary;
        }
    }
}




