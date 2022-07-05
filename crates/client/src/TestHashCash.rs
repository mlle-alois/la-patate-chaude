extern crate core;

#[cfg(test)]
mod TestHashCash {
    #[test]
    fn to_binary_can_assert() {
        let binary = to_binary('A');
        assert!("1010",binary);
    }
}