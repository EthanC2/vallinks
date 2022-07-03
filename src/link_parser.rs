use std::collections::HashSet;

use reqwest::{Client, Url, redirect};
use scraper::{Html, Selector};
use lazy_static::lazy_static;
use const_format::concatcp;
use regex::Regex;

use crate::website::Website;
use crate::link::Link;


//Constants
const APP_VERSION: &str = "0.1.0";
const USER_AGENT: &str = concatcp!("VALLINKS", "/", APP_VERSION);

pub struct LinkParser {
    pub client: reqwest::Client,
    pub websites: Vec<Website>,
    cache: HashSet<String>,
}

impl LinkParser {
    pub fn new() -> Self {
        let client = Client::builder()
                            .user_agent(USER_AGENT)
                            .redirect(redirect::Policy::limited(10))
                            .timeout(core::time::Duration::from_secs(30))
                            .build()
                            .expect("fatal: could not build HTTP client in src/link_parser::new()");

        Self {client: client, websites: Vec::new(), cache: HashSet::new() }
    }

    pub fn with_config(config: &Config) -> Self {
        let client = Client::builder()
                            .user_agent(USER_AGENT)
                            .redirect(redirect::Policy::limited(config.max_redirects))
                            .timeout(core::time::Duration::from_secs(config.timeout))
                            .build()
                            .expect("fatal: could not build HTTP client in src/link_parser::new()");

        Self {client: client, websites: Vec::new(), cache: HashSet::new() }
    }

    /* Note: a <base> tag MAY not have a 'href': https://developer.mozilla.org/en-US/docs/Web/HTML/Element/base */
    pub async fn get_links(&mut self, website: &mut Website) -> reqwest::Result<()> {
        let html = website.get_html(&self.client).await?;
        let document = Html::parse_document(&html);
        let link_selector = Selector::parse(r#"a"#).expect("could not unwrap \'a\' tag selector");

        let mut base_url: Url = website.url.clone();
        let base_selector = Selector::parse(r#"base"#).expect("could not unwrap \'base\' tag selector");;
        if let Some(base_tag) = document.select(&base_selector).nth(1) {
            if let Some(href) = base_tag.value().attr("href") {
                base_url = Url::parse(href).unwrap_or(website.url.clone()); //todo: fix redundent assignment
            }
        }

        for a_tag in document.select(&link_selector) {
            if let Some(href) = a_tag.value().attr("href") {
                if !self.cache.contains(href) {
                    let url: Url;
                    if Self::is_relative_link(href) {
                        url = base_url.join(href).unwrap();
                    } else {
                        url = Url::parse(href).unwrap();
                    }

                    self.cache.insert(href.to_owned());
                    website.links.push(Link::new(url));
                }
            }
        }

        Ok(())
    }

    //TODO: fix bug that causes 'mailto: ' to be counted as a relative link.
    fn is_relative_link(url: &str) -> bool { //TODO: make 'http_regex' static with 'lazy_static!'
        lazy_static! {
            static ref HTTP_REGEX: Regex = Regex::new(r#"^http"#).expect("failed to unwrap regex in src/link_parser");
        }

        !HTTP_REGEX.is_match(url)
    }
}




pub struct Config {
    timeout: u64,
    max_redirects: usize,
}

impl Config {
    pub fn new(timeout: u64, max_redirects: usize) -> Self {
        Self { timeout: timeout, max_redirects: max_redirects }
    }
}