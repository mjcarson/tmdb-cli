use serde::{Deserialize, de::DeserializeOwned};

use crate::{get, client};


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
  pub page: u64 ,
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
  pub (super) fn new(
    url: String,
    token: &str,
  ) -> Cursor<T> {
    // build client
    let client = client!();
    // create default review cursor
    Cursor {
      url,
      client,
      token: token.to_owned(),
      page: 0,
      params: Vec::default(),
      results: Vec::default(),
      total_pages: 0,
      total_results: 0,
    }
  }

  pub fn page(mut self, page: u64) -> Self {
    self.page = page;
    self
  }

  pub fn params(mut self, params: Vec<(String, String)>) -> Self {
    self.params = params;
    self
  }

  pub async fn next_page(mut self) -> Result<Self, reqwest::Error> {
    // increment our current page
    self.page += 1;
    // build a request using the our token and query
    let req = self.client.get(&self.url)
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
}
