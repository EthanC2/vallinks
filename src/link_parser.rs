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
    pub http_client: reqwest::Client,
    cache: HashSet<String>,
}

impl LinkParser {
    // not needed, but preserved in case
    // pub fn new() -> Self {
    //     let client = Client::builder()
    //                         .user_agent(USER_AGENT)
    //                         .redirect(redirect::Policy::limited(10))
    //                         .timeout(core::time::Duration::from_secs(30))
    //                         .build()
    //                         .expect("fatal: could not build HTTP client in src/link_parser::new()");

    //     Self {http_client: client, cache: HashSet::new() }
    // }

    pub fn with_config(max_redirects: &usize, timeout: &u64) -> Self {
        let client = Client::builder()
                            .user_agent(USER_AGENT)
                            .redirect(redirect::Policy::limited(*max_redirects))
                            .timeout(core::time::Duration::from_secs(*timeout))
                            .build()
                            .expect("fatal: could not build HTTP client in src/link_parser::with_config()");

        Self {http_client: client, cache: HashSet::new() }
    }

    pub async fn get_links(&mut self, website: &mut Website) -> reqwest::Result<()> {
        let html = website.get_html(&self.http_client).await?;
        let document = Html::parse_document(&html);
        let link_selector = Selector::parse(r#"a"#).expect("could not unwrap <a> tag selector");
        let base_url = Self::get_base_url(&document).unwrap_or(website.url.clone());

        for a_tag in document.select(&link_selector) {
            if let Some(href) = a_tag.value().attr("href") {
                if !self.cache.contains(href) {
                    let url: Url;
                    if Self::is_relative_link(href) {
                        url = base_url.join(href).unwrap();   //TODO: fix bad error handling.
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

    /* Note: a website may not have a base URL even if they have a <base> tag, in which case
       the base URL defaults the current URL. Read more at https://developer.mozilla.org/en-US/docs/Web/HTML/Element/base */
    pub fn get_base_url(document: &Html) -> Option<Url> {
        let base_selector = Selector::parse(r#"base"#).expect("could not unwrap <base> tag selector");
        if let Some(base_tag) = document.select(&base_selector).nth(1) {
            if let Some(href) = base_tag.value().attr("href") {
                return Url::parse(href).ok();
            }
        }

        None
    }

    //TODO: fix bug that causes 'mailto: ' to be counted as a relative link.
    fn is_relative_link(url: &str) -> bool {
        lazy_static! {
            static ref HTTP_REGEX: Regex = Regex::new(r#"^http"#).expect("failed to unwrap regex in src/link_parser");
        }

        !HTTP_REGEX.is_match(url)
    }
}