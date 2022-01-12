#[cfg(test)]
mod tests {

    #[test]
    fn env_test() {
        std::env::var("HEROKU_API_KEY").unwrap();
        assert_eq!(std::env::var("CARGO_TERM_COLOR").unwrap(), "always");
        assert_eq!(std::env::var("SOMEVAR").unwrap(), "this_works");
    }
    #[test]
    fn fail_test() {
        panic!();
    }
}
// should triggerqwdqwdqwdqwdqwdasdasdsadsqdqdqASDASDASDASDASDASDqwdqwdqwd
fn main() {
    println!("Hello, world!");
}
