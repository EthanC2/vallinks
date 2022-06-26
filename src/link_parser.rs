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
                            .expect("fatal: could not build client in src/link_parser");

        Self {client: client, websites: Vec::new(), cache: HashSet::new() }
    }

    pub async fn get_links(&mut self, website: &mut Website) -> reqwest::Result<()> {
        let html = website.get_html(&self.client).await?;
        let document = Html::parse_document(&html);
        let link_selector = Selector::parse(r#"a"#).unwrap();

        let mut base_url: Url = website.url.clone();
        let base_selector = Selector::parse(r#"base"#).unwrap();
        if let Some(base_tag) = document.select(&base_selector).nth(1) {
            if let Some(href) = base_tag.value().attr("href") {
                base_url = Url::parse(href).unwrap_or(website.url.clone()); //todo: fix redundent assignment
            }
        }

        for a_tag in document.select(&link_selector) {
            if a_tag.value().attr("href").is_some() {   //todo: why doesn't 'if let a_tag.value().attr("href")' work here?
                let href = a_tag.value().attr("href").unwrap();
                
                if !self.cache.contains(href) {
                    let url: Url;

                    if Self::is_relative_link(href) {
                        url = base_url.join(href).unwrap();
                    } else {
                        url = Url::parse(href).unwrap();
                    }

                    self.cache.insert(String::from(url.as_str()));
                    let link = Link::new(&self.client, url).await;
                    website.links.push(link);
                }
            }
        }

        Ok(())
    }

    fn is_relative_link(url: &str) -> bool { //TODO: make 'http_regex' static with 'lazy_static!'
        lazy_static! {
            static ref HTTP_REGEX: Regex = Regex::new(r#"^http"#).expect("failed to unwrap regex in src/link_parser");
        }

        !HTTP_REGEX.is_match(url)
    }
}