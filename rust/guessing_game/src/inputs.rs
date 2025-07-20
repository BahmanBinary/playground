pub struct Guess {
    pub value: u8,
}

impl Guess {
    pub fn new(value: u8) -> Result<Self, String> {
        Self::validate(value)?;

        Ok(Self { value })
    }

    fn validate(value: u8) -> Result<(), String> {
        if value < 1 {
            return Err(String::from("Value is less than 1!"));
        } else if value > 100 {
            return Err(String::from("Value is more than 100!"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Guess;

    #[test]
    fn test_guess_validation() -> Result<(), String> {
        let guess = Guess::new(0);
        assert!(guess.is_err());

        let guess = Guess::new(101);
        assert!(guess.is_err());

        let number = 100u8;
        let guess = Guess::new(number)?;

        assert_eq!(guess.value, number);

        Ok(())
    }
}
