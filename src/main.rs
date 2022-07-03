mod link_parser;
mod website;
mod link;

use link_parser::{LinkParser, Config};
use website::Website;

use clap::{Command, Arg, ArgAction, value_parser};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Broken Link Finder (BLF)")
                    .version("1.0")
                    .author("Ethan Cox (ethanrcox@protonmail.com)")
                    .about("Parses websites for broken <a> tags")
                    .arg(Arg::new("website")
                        .short('w')
                        .long("website")
                        .help("URL of website(s) to parse")
                        .takes_value(true)
                        .multiple_values(true)
                        .value_name("URL")
                        .required(true)
                        .action(ArgAction::Set)
                    )
                    .arg(Arg::new("timeout")
                        .short('t')
                        .long("timeout")
                        .help("HTTP request timeout in seconds")
                        .takes_value(true)
                        .value_name("TIMEOUT")
                        .value_parser(value_parser!(u64))
                        .default_value("30")
                        .action(ArgAction::Set)
                    )
                    .arg(Arg::new("max-redirects")
                        .short('r')
                        .long("max-redirects")
                        .help("Maximum amount of redirects per HTTP request")
                        .takes_value(true)
                        .value_name("MAXIMUM REDIRECTS")
                        .value_parser(value_parser!(usize))
                        .default_value("10")
                        .action(ArgAction::Set)
                    )
                    .get_matches();

    let websites = matches.get_many::<String>("website")
                                        .expect("required")
                                        .map(|url| Website::new(url.into()))
                                        .collect::<Vec<Website>>();
    let config = Config::new(
        *matches.get_one("timeout").expect("default"),
        *matches.get_one("max-redirects").expect("default")
    );

    let mut parser = LinkParser::with_config(&config); 
    let mut website = Website::new(&"https://ethancox.dev/".to_string());

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
