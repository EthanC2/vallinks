mod link_parser;
mod website;
mod link;

use link_parser::*;
use website::Website;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let parser = LinkParser::new(); 
    let mut website = Website::new("https://open.kattis.com/problems?order=problem_difficulty&show_solved=off&show_tried=off&show_untried=on");

    parser.get_links(&mut website).await?;
    for link in website.links.iter() {
        println!("[{}] {}", link.status_code, link.href.as_str());
    }

    Ok(())
}
