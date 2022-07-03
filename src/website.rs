use reqwest::{Client, Url, Result};

use crate::link::Link;

#[derive(Debug)]
pub struct Website {
    pub url: Url, 
    pub links: Vec<Link>,
}

impl Website {
    pub fn new<TStr>(url: &TStr) -> Self where TStr: AsRef<str> {
        let url = Url::parse(url.as_ref())
                        .expect(&format!("fatal: invalid website url format: {}", url.as_ref()));
        
        Self {url: url, links: Vec::new()}
    }

    pub async fn get_html(&self, client: &Client) -> Result<String> {
        let url = self.url.clone();    //why is this by value?
        let response = client.get(url).send().await?;
        let text = response.text().await?;

        Ok(text)
    }
}