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
    let mut website = Website::new("https://sematext.com/blog/linux-logs/");

    parser.get_links(&mut website).await?;
    let futures = website.links.iter_mut().map(|link| link.get_status(&parser.client));
    futures::future::join_all(futures).await;

    for link in website.links.iter() {
        match &link.response {
            Some(Ok(res)) => println!("[{}] {}", res.status(), link.href.as_str()),
            Some(Err(error)) => eprintln!("error: {}", error),
            None => eprintln!("error: link did not contain href attribute"),
        }
    }

    Ok(())
}
