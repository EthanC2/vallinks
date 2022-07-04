mod link_parser;
mod website;
mod link;

use link_parser::LinkParser;
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
                    .arg(Arg::new("quiet")
                        .short('q')
                        .long("quiet")
                        .help("Suppresses logging links, reporting only broken links")
                        .default_value("true")
                        .action(ArgAction::SetFalse)
                    )
                    .get_matches();

    // 1. Initialization of websites to parse and link parser configuration from cmdline params
    //TODO: clean up cmdline param initialization
    let timeout = matches.get_one("timeout").expect("default");
    let max_redirects = matches.get_one("max-redirects").expect("default");
    let print_good_links = *matches.get_one("quiet").expect("default");

    let mut websites = matches.get_many::<String>("website")
                                        .expect("required")
                                        .map(|url| Website::new(url))
                                        .collect::<Vec<Website>>();


    let mut parser = LinkParser::with_config(max_redirects, timeout); 


    //2. Parse each website in the list of websites
    for website in websites.iter_mut() {
        if parser.get_links(website).await.is_ok() {
            let futures = website.links
                                                                                .iter_mut()
                                                                                .map(|link|link.get_status(&parser.http_client));
            futures::future::join_all(futures).await;

            for link in website.links.iter() {
                match &link.response {
                    Some(Ok(res)) if res.status().is_success() => if print_good_links {println!("[{}] {} {}", res.status(), website.url, link.href)},
                    Some(Ok(res)) => println!("[{}] {} {}", res.status(), website.url, link.href),
                    Some(Err(error)) => eprintln!("error on : {}", error),
                    None => eprintln!("error: link did not contain href attribute"),  //TODO: fix dead code
                }
            }
        } else {
            eprintln!("ERROR: could not reach {}", website.url);
        }
        println!();
    }

    Ok(())
}
