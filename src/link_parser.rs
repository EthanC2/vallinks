use std::collections; //std::collections::HashSet prevents re-visiting links

use reqwest::{Client, Url, redirect};
use scraper::{Html, Selector};
use regex::Regex;
use const_format::concatcp;

use crate::website::Website;
use crate::link::Link;


//Constants
const APP_VERSION: &str = "0.1.0";
const USER_AGENT: &str = concatcp!("VALLINKS", "/", APP_VERSION);

pub struct LinkParser {
    pub client: reqwest::Client,
    pub websites: Vec<Website>,
}

impl LinkParser {
    pub fn new() -> Self {
        let client = Client::builder()
                            .user_agent(USER_AGENT)
                            .redirect(redirect::Policy::limited(10))
                            .timeout(core::time::Duration::from_secs(30))
                            .build()
                            .expect("fatal: could not build client in src/link_parser");

        Self {client: client, websites: Vec::new()}
    }

    pub async fn get_links(&self, website: &mut Website) -> reqwest::Result<()> {
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
            if a_tag.value().attr("href").is_some() {
                let href = a_tag.value().attr("href").unwrap();
                let mut url: Url;

                if Self::is_relative_link(href) {
                    url = base_url.join(href).unwrap();
                } else {
                    url = Url::parse(href).unwrap();
                }

                let link = Link::new(&self.client, url).await;
                website.links.push(link);
            }
        }

        Ok(())
    }

    fn is_relative_link(url: &str) -> bool { //TODO: make 'http_regex' static with 'lazy_static!'
        let http_regex: Regex = Regex::new(r#"^http"#).expect("failed to unwrap regex in src/link_parser");
        !http_regex.is_match(url)
    }
}