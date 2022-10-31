pub struct Frog {

}

impl Frog {
    pub fn test() {
        println!("FROG CORE HERE");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
