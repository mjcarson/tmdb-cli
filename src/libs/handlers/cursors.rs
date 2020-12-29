use serde::{de::DeserializeOwned, Deserialize};

use crate::{client, get};

/// A cursor page that we will use to hydrate our cursor
#[derive(Deserialize)]
struct CursorPage<T> {
    /// The current page of this search
    pub page: u64,
    /// The currently retreived results
    pub results: Vec<T>,
    /// The total number of pages that exist
    pub total_pages: i64,
    /// The total number of results that exist
    pub total_results: i64,
}

/// A cursor for objects of some type
#[derive(Clone, Debug)]
pub struct Cursor<T: DeserializeOwned> {
    /// The url to use
    pub url: String,
    /// A reqwest client object
    pub client: reqwest::Client,
    /// A token to use when authenticating
    pub token: String,
    /// The page the cursor is currently at
    pub page: u64,
    /// The query params to use when paging through results with this cursor
    pub params: Vec<(String, String)>,
    /// The currently retreived results
    pub results: Vec<T>,
    /// The total number of pages that exist
    pub total_pages: i64,
    /// The total number of results that exist
    pub total_results: i64,
}

impl<T: DeserializeOwned> Cursor<T> {
    /// Creates a new unhydrated cursor
    ///
    /// # Arguments
    ///
    /// * `url` - The url this cursor should query
    /// * `token` - The token used for authentication when querying TMDB
    pub(super) fn new(url: String, token: &str) -> Cursor<T> {
        // build client
        let client = client!();
        // create default review cursor
        Cursor {
            url,
            client,
            token: token.to_owned(),
            page: 1,
            params: Vec::default(),
            results: Vec::default(),
            total_pages: 0,
            total_results: 0,
        }
    }

    /// Changes this cursors current page value
    ///
    /// This does not change the data the cursor currently has loaded.
    ///
    /// # Arguments
    ///
    /// * `page` - The page to set
    pub fn page(mut self, page: u64) -> Self {
        self.page = page;
        self
    }

    /// Adds a single url parameter
    ///
    /// This does not change the data the cursor currently has loaded and is purely additive.
    ///
    /// # Arguments
    ///
    /// * `params` - The params to set
    pub fn param<U: Into<String>>(mut self, key: U, value: U) -> Self {
        self.params.push((key.into(), value.into()));
        self
    }

    /// Adds a vector of url parameter
    ///
    /// This does not change the data the cursor currently has loaded and is purely additive.
    ///
    /// # Arguments
    ///
    /// * `params` - The params to set
    pub fn params(mut self, params: Vec<(String, String)>) -> Self {
        self.params.extend(params);
        self
    }

    /// Load the data for the current page and params set
    #[syncwrap::wrap]
    pub async fn exec(mut self) -> Result<Self, reqwest::Error> {
        // build a request using the our token and query
        let req = self
            .client
            .get(&self.url)
            .query(&[("api_key", &self.token)])
            .query(&[("page", &self.page)])
            .query(&self.params);
        // send request and build objects from the response to update our cursor
        let data = get!(self, req)?.json::<CursorPage<T>>().await?;
        // update our cursor
        self.results = data.results;
        self.total_pages = data.total_pages;
        self.total_results = data.total_results;
        Ok(self)
    }

    /// Load the data for the next page overwritting all data currently loaded
    #[syncwrap::wrap]
    pub async fn next_page(mut self) -> Result<Self, reqwest::Error> {
        // increment our current page
        self.page += 1;
        // load the data for the newly set page
        self.exec().await
    }
}
