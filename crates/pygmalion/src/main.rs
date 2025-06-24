use pygma_parser;
use pygma_render;

fn main() {
    println!("Hello, world!");

    match pygma_parser::parse_text("Hello, Pygmalion!") {
        Ok(parsed) => println!("Parser result: {}", parsed),
        Err(e) => println!("Parser error: {}", e),
    }

    match pygma_render::render_content("Hello, Pygmalion!") {
        Ok(rendered) => println!("Renderer result: {}", rendered),
        Err(e) => println!("Renderer error: {}", e),
    }
}
