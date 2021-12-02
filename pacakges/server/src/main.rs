#[cfg(test)]
mod tests {

    #[test]
    fn env_test() {
        std::env::var("HEROKU_API_KEY").unwrap();
        assert_eq!(std::env::var("CARGO_TERM_COLOR").unwrap(), "always");
    }
}

fn main() {
    println!("Hello, world!");
}
