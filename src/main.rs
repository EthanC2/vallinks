mod link_parser;
mod website;
mod link;

use link_parser::*;
use website::Website;

use std::collections::HashSet;
use core::time::Duration;

use clap::Parser;
use reqwest::*;
use scraper::{Html, Selector};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let parser = LinkParser::new(); 
    let mut website = Website::new("https://www.spanishdict.com/");

    parser.get_links(&mut website).await?;
    for link in website.links.iter() {
        println!("{:?}", link);
    }

    Ok(())
}
