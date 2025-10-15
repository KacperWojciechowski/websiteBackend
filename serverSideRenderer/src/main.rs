use kuchikiki::parse_html;
use kuchikiki::traits::*;

fn parse_my_html(input : &str) -> std::result::Result<kuchikiki::NodeRef, std::io::Error> {
    return parse_html().from_utf8().from_file(input);
}

fn main() {
    println!("HTML Parser!");

    let parsed = parse_my_html("../../websiteFrontend/index.html");

    println!("Parsed: {}", parsed.unwrap().to_string());

    
}
