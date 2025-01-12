pub fn return_hello() -> String {
    "Hello".to_string()
}

fn main() {
    println!("{}", return_hello());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_return_hello() {
        assert_eq!(return_hello(), "Hello");
    }
}
