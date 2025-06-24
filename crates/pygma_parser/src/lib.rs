pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse(&self, input: &str) -> Result<String, String> {
        Ok(format!("Parsed: {}", input))
    }
}

pub fn parse_text(input: &str) -> Result<String, String> {
    let parser = Parser::new();
    parser.parse(input)
}
