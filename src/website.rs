use reqwest::{Client, Result};
use url::Url;

use crate::link::Link;

#[derive(Debug)]
pub struct Website {
    pub url: Url, 
    pub links: Vec<Link>,
}

impl Website {
    pub fn new(url: &str) -> Self {
        // let http_regex = Regex::new(r#"^https?://"#).unwrap(); 
        // if !http_regex.is_match(url) {
        //     let url = format!("http://{}", url).as_str();
        // }
        let url = Url::parse(url)
                        .expect(&format!("fatal: invalid website url format: {}", url));
        
        Self {url: url, links: Vec::new()}
    }

    pub async fn get_html(&self, client: &Client) -> Result<String> {
        let url = self.url.clone();    //why is this by value?
        let response = client.get(url).send().await?;
        let text = response.text().await?;

        Ok(text)
    }
}