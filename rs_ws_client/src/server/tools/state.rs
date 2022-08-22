use serde_diff::SerdeDiff;

pub struct State<T: SerdeDiff> {
    current_state: T,
    last_state: T,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
