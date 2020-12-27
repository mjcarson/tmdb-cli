use super::core::Core;
use crate::libs::models::{MovieList, MovieDetails};
use crate::{client, opt_param, get};

/// Movie search cursor
#[derive(Clone)]
pub struct MovieSearch<'a> {
  /// The url to use
  url: String,
  /// The handler being used to perform this search
  handler: &'a Movies, 
  /// The current page of this search
  pub page: u64,
  /// The query in use
  pub query: String,
  /// The region movies returned must be from
  pub region: Option<String>,
  /// The year movies returned must have been released in
  pub year: Option<String>,
  /// The primary year movies returned must have been released in
  pub primary_year: Option<String>,
  /// The language movies returned must be in
  pub language: Option<String>,
  /// Whether adult movies should be returned
  pub adult: bool,
}

impl<'a> MovieSearch<'a> {
  /// Search for movies on the currently selected page
  ///
  /// # Examples
  ///
  /// ```
  /// pub use tmdb_cli::Client;
  ///
  /// # #[tokio::main]
  /// # async fn main() {
  /// // build a client
  /// let tmdb = Client::from_env();
  /// // serach for a movie
  /// let search = tmdb.movies.search("13 Hours")
  ///   .year(2016)
  ///   .exec()
  ///   .await;
  /// # assert!(search.is_ok())
  /// # }
  /// ```
  #[syncwrap::wrap]
  pub async fn exec(&self) -> Result<MovieList, reqwest::Error> {
    // cast page to a string
    let page = self.page.to_string();
    let adult = self.adult.to_string();
    // build the url query params
    let mut params = Vec::with_capacity(3);
    params.push(("api_key", &self.handler.token));
    params.push(("query", &self.query));
    params.push(("page", &page));
    params.push(("adult", &adult));
    // add any optional params if they exist
    opt_param!(params, "region", self.region);
    opt_param!(params, "year", self.year);
    opt_param!(params, "primary_year", self.primary_year);
    opt_param!(params, "language", self.language);
    // build a request using the our token and query
    let req = self.handler.client.get(&self.url).query(&params);
    // send request and build a MovieList from the response
    get!(self.handler, req)?.json::<MovieList>().await
  }

  /// Change the current page of our search
  ///
  /// # Arguments
  ///
  /// * `page` - The page to query when this search is executed
  pub fn page(mut self, page: u64) -> Self {
    self.page = page;
    self
  }

  /// Sets the region you want movies returned from this search to be from
  ///
  /// # Arguments
  ///
  /// * `region` - The region to filter movies by
  pub fn region(mut self, region: String) -> Self {
    self.region = Some(region);
    self
  }

  /// Sets the year movies returned from this search must have been released on
  ///
  /// # Arguments
  ///
  /// * `year` - The year of release to filter on
  pub fn year(mut self, year: u64) -> Self {
    self.year = Some(year.to_string());
    self
  }
  
  /// Sets the primary year movies returned from this search must have been released on
  ///
  /// # Arguments
  ///
  /// * `primary_year` - The primary year of release to filter on
  pub fn primary_year(mut self, primary_year: u64) -> Self {
    self.primary_year = Some(primary_year.to_string());
    self
  }

  /// Sets the language that movies returned by this search must be in
  ///
  /// # Arguments
  ///
  /// * `lang` - The language to filter on
  pub fn language(mut self, lang: String) -> Self {
    self.language = Some(lang);
    self
  }
  
  /// Allows adult movies to be returned by this search
  pub fn adult(mut self) -> Self {
    self.adult = true;
    self
  }
}

/// Handlers for Movie focused routes
#[derive(Clone)]
pub struct Movies {
  /// The URL/ip to reach tmdb at
  host: String,
  /// A reqwest client object
  client: reqwest::Client,
  /// A token to use when authenticating
  token: String,
}

impl Movies{
  /// Create a new Movies handler
  ///
  /// # Arguments
  ///
  /// * `host` - The host/url tmdb is at
  /// * `token` - The token to use when authenticating
  pub fn new(host: &str, token: &str) -> Self {
    // build client
    let client = client!();
    // build movies handler
    Movies {
      host: host.to_owned(),
      client,
      token: token.to_owned(),
    }
  }

  /// Search for a movie
  ///
  /// # Arguments
  ///
  /// * `query` - The query to use when searching
  ///
  /// # Examples
  ///
  /// ```
  /// pub use tmdb_cli::Client;
  ///
  /// # #[tokio::main]
  /// # async fn main() {
  /// // build a client
  /// let tmdb = Client::from_env();
  /// // serach for a movie
  /// let search = tmdb.movies.search("13 Hours")
  ///   .year(2016)
  ///   .exec()
  ///   .await;
  /// # assert!(search.is_ok())
  /// # }
  /// ```
  pub fn search<T: Into<String>>(&self, query: T) -> MovieSearch {
    MovieSearch {
      url: format!("{}/3/search/movie", &self.host),
      handler: self,
      // start at page 1 because tmdb doesn't use 0 based indexes
      page: 1,
      query: query.into(),
      region: None,
      year: None,
      primary_year: None,
      language: None,
      adult: false,
    } 
  }

  /// Get details on a movie by id
  ///
  /// # Arguments
  ///
  /// * `id` - The ID of the movie to retrieve details on
  ///
  /// # Examples
  ///
  /// ```
  /// pub use tmdb_cli::Client;
  ///
  /// # #[tokio::main]
  /// # async fn main() {
  /// // build a client
  /// let tmdb = Client::from_env();
  /// // serach for a movie
  /// let search = tmdb.movies.details(157336).await;
  /// # assert!(search.is_ok())
  /// # }
  /// ```
  pub async fn details(&self, id: i64) -> Result<MovieDetails, reqwest::Error> {
    // build url to query
    let url = format!("{}/3/movie/{}", &self.host, id);
    // build a request using the our token and query
    let req = self.client.get(&url)
      .query(&[("api_key", &self.token)]);
    // send request and build a MovieList from the response
    get!(self, req)?.json::<MovieDetails>().await
  }
}
