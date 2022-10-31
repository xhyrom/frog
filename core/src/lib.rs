pub struct FrogCore {
}

impl FrogCore {
    pub fn test() {
        println!("FROG CORE HERE");
    }

    pub fn init(name: String, directory: String, language: String) {
        
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
