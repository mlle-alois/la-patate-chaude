extern crate core;

#[cfg(test)]
mod TestHashCash {
    use crate::get_type;
    use crate::hash_cash::{convert_to_binary_from_hex, create_seed, determine_complexity, format_seed_and_message, generate_hash, is_hashcode_valid, to_binary};

    #[test]
    fn to_binary_can_assert() {
        let binary = to_binary('A');
        assert!("1010","{}",binary.to_string());
        let binary = to_binary('6');
        assert!("0110","{}",binary.to_string());
    }
    #[test]
    fn is_hashcode_valid_assert(){
        let verif_true = is_hashcode_valid("00006CAF49510986FF0E25C85F2E3088".to_string(),16);
        assert!(true,"{}",verif_true);
        let verif_false = is_hashcode_valid("10006CAF49510986FF0E25C85F2E3088".to_string(),16);
        assert!(false,"{}",verif_false);
    }
    #[test]
    fn convert_to_binary_from_hex_assert(){
        let binary = convert_to_binary_from_hex("00".to_string());
        assert!("00000000","{}",binary);
        let binary = convert_to_binary_from_hex("AB".to_string());
        assert!("10101011","{}",binary);
    }
    #[test]
    fn create_seed_assert(){
        let seed = create_seed(182647);
        assert!("00006CAF49510986FF0E25C85F2E3088","{}",seed);
    }
    #[test]
    fn generate_hash_assert(){
        let hash = generate_hash(9,"hello");
        assert!("00441745D9BDF8E5D3C7872AC9DBB2C3","{}",hash);
    }
    #[test]
    fn format_seed_and_message_assert(){
        let msg = format_seed_and_message(52,"hello");
        assert!("000000000000034Chello","{}",msg);
    }
    #[test]
    fn determine_complexity_assert(){
        let val = determine_complexity("00006CAF49510986FF0E25C85F2E3088");
        assert!(16,"{}",val);
    }
 /*   #[test]
    fn get_message_type_assert(){
       // let msgType = get_type(Message::Hello);
        //assert!("Hello","{}",msgType);
        let msgType = get_type(Message::EndOfGame);
        assert!("EndOfGame","{}",msgType);
        let msgType = get_type(Message::Challenge);
        assert!("Challenge","{}",msgType);
    }*/
}