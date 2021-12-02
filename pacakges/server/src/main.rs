

#[cfg(test)]
mod tests {

    #[test]
    fn env_test() {
        println!("{:#?}", std::env::var("HEROKU_API_KEY"));
    }
}


fn main() {
    println!("Hello, world!");
}

