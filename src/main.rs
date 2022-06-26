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
    let mut parser = LinkParser::new(); 
    let mut website = Website::new("https://www.gatevidyalay.com/graphs-types-of-graphs/");

    parser.get_links(&mut website).await?;
    for link in website.links.iter() {
        println!("[{}] {}", link.status_code, link.href.as_str());
    }

    Ok(())
}
