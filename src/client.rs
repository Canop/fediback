use {
    crate::*,
    reqwest::{blocking, header},
    serde::de::DeserializeOwned,
};

pub struct Client {
    requester: blocking::Client,
    pub verbose: bool,
}

const MAX_QUERIES_PER_LIST: usize = 100;

impl Client {

    pub fn new() -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/activity+json"),
        );
        let requester = blocking::Client::builder()
            .default_headers(headers)
            .user_agent("fediback/0.1")
            .build()?;
        let verbose = false;
        Ok(Self { requester, verbose })
    }

    pub fn get<D>(&self, url: &str) -> Result<D>
    where
        D: DeserializeOwned,
    {
        if self.verbose {
            eprintln!("querying {}", url);
        }
        Ok(self.requester.get(url).send()?.json()?)
    }

    pub fn get_items<S, Item>(
        &self,
        url: S,
    ) -> Result<Vec<Item>>
        where
            S: Into<String>,
            Item: DeserializeOwned,
    {
        let mut url = url.into();
        let mut all = Vec::new();
        let mut queries = 0;
        loop {
            let mut page: Page<Item> = self.get(&url)?;
            queries += 1;
            if let Some(items) = &mut page.ordered_items {
                all.append(items);
            }
            if let Some(next) = page.next.take() {
                url = next;
            } else if let Some(first) = page.first.take() {
                url = first;
            } else {
                break;
            }
            if queries >= MAX_QUERIES_PER_LIST {
                warn!("too many queries");
                if self.verbose {
                    warn!("too many queries");
                }
                break;
            }
        }
        Ok(all)
    }

}
