extern crate core;

#[cfg(test)]
mod TestHashCash {
    use shared::models::challenge::Challenge;
    use shared::models::challenge_answer::ChallengeAnswer::MD5HashCash;
    use shared::models::challenge_result::ChallengeResult;
    use shared::models::md5hash_cash_input::MD5HashCashInput;
    use shared::models::message::Message;
    use shared::models::public_player::PublicPlayer;
    use shared::models::subscribe::Subscribe;
    use shared::models::welcome::Welcome;
    use crate::get_type;
    use crate::hash_cash::{convert_to_binary_from_hex, create_seed,generate_random_string, serialize_message, generate_hash, is_hashcode_valid, to_binary};

    #[test]
    fn serialize_message_assert() {
        let msg = serialize_message(&Message::Hello);
        println!("ok:{msg}");
        assert_eq!("\"Hello\"",msg);
    }
    #[test]
    fn generate_random_string_assert() {
        let ch = generate_random_string(6);
        assert_eq!(6,ch.len());
    }
    #[test]
    fn to_binary_can_assert() {
        let binary = to_binary('A');
        assert_eq!("1010",binary);
        let binary = to_binary('6');
        assert_eq!("0110",binary);
    }
    #[test]
    fn is_hashcode_valid_assert(){
        let verif_true = is_hashcode_valid("00006CAF49510986FF0E25C85F2E3088".to_string(),16);
        assert_eq!(true,verif_true);
        let verif_false = is_hashcode_valid("10006CAF49510986FF0E25C85F2E3088".to_string(),16);
        assert_eq!(false,verif_false);
    }
    #[test]
    fn convert_to_binary_from_hex_assert(){
        let binary = convert_to_binary_from_hex("00".to_string());
        assert_eq!("00000000",binary);
        let binary = convert_to_binary_from_hex("AB".to_string());
        assert_eq!("10101011",binary);
    }
    #[test]
    fn create_seed_assert(){
        let seed = create_seed(52);
        assert_eq!("0000000000000034",seed);
    }
    #[test]
    fn generate_hash_assert(){
        let hash = generate_hash(9,"hello");
       // assert_eq!("00441745D9BDF8E5D3C7872AC9DBB2C3",hash);
    }
    #[test]
    fn get_message_type_assert(){
        let msg_type = get_type(&Message::Hello);
        assert_eq!("Hello",msg_type);
        let msg_type = get_type(&Message::Welcome(Welcome{version: 2 as u8 }));
        assert_eq!("Welcome",msg_type);
        let msg_type = get_type(&Message::Subscribe(Subscribe{name: "test".parse().unwrap() }));
        assert_eq!("Subscribe",msg_type);
    }
}