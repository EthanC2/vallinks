use reqwest::{Client, Url, Response, Result};
use backoff::ExponentialBackoff;

#[derive(Debug)]
pub struct Link {
    pub href: Url,
    pub response: Option<Result<Response>>,
}

impl Link {
    pub fn new(url: Url) -> Self {                                    
        Link { href: url, response: None }
    }

    pub async fn get_status(&mut self, client: &Client) {
        let url = self.href.clone();
        let resp = client.head(url).send().await;
        self.response = Some(resp);
    }
}