use pygma_parser;

pub struct Renderer {
    // レンダラーの状態を管理
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn render(&self, parsed_content: &str) -> Result<String, String> {
        Ok(format!("Rendered: {}", parsed_content))
    }
}

pub fn render_content(input: &str) -> Result<String, String> {
    let parsed = pygma_parser::parse_text(input)?;
    let renderer = Renderer::new();
    renderer.render(&parsed)
}

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
