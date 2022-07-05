use reqwest::{Client, Url, Result, IntoUrl};

use crate::link::Link;

pub struct Website {
    pub url: Url, 
    pub links: Vec<Link>,
}

impl Website {
    pub fn new<T: IntoUrl>(url: T) -> Self {
        let url = url.into_url().unwrap(); //TODO: fix bad error handling
        Self {url: url, links: Vec::new()}
    }

    pub async fn get_html(&self, client: &Client) -> Result<String> {
        let url = self.url.clone();    //why is this by value?
        let response = client.get(url).send().await?;
        let text = response.text().await?;

        Ok(text)
    }
}