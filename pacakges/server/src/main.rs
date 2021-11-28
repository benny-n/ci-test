fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    fn take_vec(asd: &Vec<u8>) {
        println!("{:#?}", asd);
    }

    #[test]
    fn fail_test() {
        let x = if 5 == 5 { 5 } else { 5 };
        take_vec(&vec![]);
        assert_eq!(x, 5);
        assert!(false);
    }
}
