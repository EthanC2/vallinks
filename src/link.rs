use reqwest::{Client, Url, StatusCode};

#[derive(Debug)]
pub struct Link {
    pub href: Url,
    pub status_code: StatusCode,
}

impl Link {
    pub async fn new(client: &Client, url: Url) -> Self {
        let status = client.head(url.clone())
                                    .send()
                                    .await
                                    .unwrap()
                                    .status(); 
                                    
        Link { href: url, status_code: status }
    }
}