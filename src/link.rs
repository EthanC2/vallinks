use reqwest::{Client, Url, Response, Result};
use tokio_retry::Retry;
use tokio_retry::strategy::{ExponentialBackoff, jitter};

pub struct Link {
    pub href: Url,
    pub response: Option<Result<Response>>,
}

impl Link {
    pub fn new(url: Url) -> Self {                                    
        Link { href: url, response: None }
    }

    pub async fn get_status(&mut self, client: &Client) {
        let retry_stategy = ExponentialBackoff::from_millis(10)
                                    .map(jitter)
                                    .take(3);

        let action = || async {
            let url = self.href.clone();       //NOTE: must re-copy each time because 'action' cannot take a param
            client.head(url).send().await
        };

        let response = Retry::spawn(retry_stategy, action).await; 
        self.response = Some(response);
    }
}