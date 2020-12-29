use super::Cursor;
use crate::libs::models::{Credits, Show, ShowDetails, Review};
use crate::{client, get, opt_param};

/// Show search cursor
#[derive(Clone)]
pub struct ShowSearch<'a> {
    /// The url to use
    url: String,
    /// The handler being used to perform this search
    handler: &'a Tv,
    /// The current page of this search
    pub page: u64,
    /// The query in use
    pub query: String,
    /// The language that shows returned must be in
    pub language: Option<String>,
    /// The year that shows returned must have been first aired in
    pub year: Option<String>,
    /// Whether adult shows should be returned
    pub adult: bool,
}

impl<'a> ShowSearch<'a> {
    /// Search for shows on the currently selected page
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
    /// // serach for a show
    /// let search = tmdb.tv.search("Red Vs. Blue")
    ///   .language("en-us")
    ///   .year(2003)
    ///   .exec()
    ///   .await;
    /// # assert!(search.is_ok())
    /// # }
    /// ```
    #[syncwrap::wrap]
    pub async fn exec(mut self) -> Result<Cursor<Show>, reqwest::Error> {
        // cast page to a string
        let adult = self.adult.to_string();
        // build the url query params
        let mut params: Vec<(String, String)> = Vec::with_capacity(2);
        params.push(("query".into(), self.query));
        params.push(("include_adult".into(), adult));
        // add any optional params if they exist
        opt_param!(params, "language", self.language);
        opt_param!(params, "first_air_date_year", self.year);
        // build cursor for this search
        Cursor::new(self.url, &self.handler.token)
            .page(self.page)
            .params(params)
            .next_page()
            .await
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

    /// Sets the year shows returned from this search must have first aired in
    ///
    /// # Arguments
    ///
    /// * `year` - The year of release to filter on
    pub fn year(mut self, year: u64) -> Self {
        self.year = Some(year.to_string());
        self
    }

    /// Sets the language that shows returned by this search must be in
    ///
    /// # Arguments
    ///
    /// * `lang` - The language to filter on
    pub fn language<T: Into<String>>(mut self, lang: T) -> Self {
        self.language = Some(lang.into());
        self
    }

    /// Allows adult shows to be returned by this search
    pub fn adult(mut self) -> Self {
        self.adult = true;
        self
    }
}

/// Handlers for TV show focused routes
#[derive(Clone)]
pub struct Tv {
    /// The URL/ip to reach tmdb at
    host: String,
    /// A reqwest client object
    pub client: reqwest::Client,
    /// A token to use when authenticating
    pub token: String,
}

impl Tv {
    /// Create a new TV show handler
    ///
    /// # Arguments
    ///
    /// * `host` - The host/url tmdb is at
    /// * `token` - The token to use when authenticating
    pub fn new(host: &str, token: &str) -> Self {
        // build client
        let client = client!();
        // build shows handler
        Tv {
            host: host.to_owned(),
            client,
            token: token.to_owned(),
        }
    }

    /// Search for a show
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
    /// // serach for a show
    /// let search = tmdb.tv.search("Red Vs. Blue")
    ///   .year(2003)
    ///   .exec()
    ///   .await;
    /// # assert!(search.is_ok())
    /// # }
    /// ```
    pub fn search<T: Into<String>>(&self, query: T) -> ShowSearch {
        ShowSearch {
            url: format!("{}/3/search/tv", &self.host),
            handler: self,
            // start at page 1 because tmdb doesn't use 0 based indexes
            page: 1,
            query: query.into(),
            year: None,
            language: None,
            adult: false,
        }
    }

    /// Get details on a show by id
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the show to retrieve details on
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
    /// // serach for a show
    /// let search = tmdb.tv.details(39373).await;
    /// # assert!(search.is_ok())
    /// # }
    /// ```
    #[syncwrap::wrap]
    pub async fn details(&self, id: i64) -> Result<ShowDetails, reqwest::Error> {
        // build url to query
        let url = format!("{}/3/tv/{}", &self.host, id);
        // build a request using the our token and query
        let req = self.client.get(&url).query(&[("api_key", &self.token)]);
        // send request and build a ShowDetails object from the response
        get!(self, req)?.json::<ShowDetails>().await
    }

    /// Get the credis for a show by id
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the show to retrieve the credits for
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
    /// // get the credits for a show
    /// let credits = tmdb.tv.credits(39373).await;
    /// # assert!(credits.is_ok())
    /// # }
    /// ```
    #[syncwrap::wrap]
    pub async fn credits(&self, id: i64) -> Result<Credits, reqwest::Error> {
        // build url to query
        let url = format!("{}/3/tv/{}/credits", &self.host, id);
        // build a request using the our token and query
        let req = self.client.get(&url).query(&[("api_key", &self.token)]);
        // send request and build a Credits object from the response
        get!(self, req)?.json::<Credits>().await
    }

    /// Builds a cursor for reviews for a tv show
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the show to retrieve reviews for
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
    /// // get the reviews for a show
    /// let reviews = tmdb.tv.reviews(39373).exec().await;
    /// # assert!(reviews.is_ok())
    /// # }
    /// ```
    pub fn reviews(&self, id: i64) -> Cursor<Review> {
        // build the url to query
        let url = format!("{}/3/tv/{}/reviews", &self.host, id);
        // build our cursor
        Cursor::new(url, &self.token)
    }

    /// Builds a cursor for shows to recommend based another tv show
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the show to base our recommendations on
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
    /// // get the recommendations for a show
    /// let recommendations = tmdb.tv.recommendations(39373).exec().await;
    /// # assert!(recommendations.is_ok())
    /// # }
    /// ```
    pub fn recommendations(&self, id: i64) -> Cursor<Show> {
        // build the url to query
        let url = format!("{}/3/tv/{}/recommendations", &self.host, id);
        // build our cursor
        Cursor::new(url, &self.token)
    }

    /// Builds a cursor for shows that are similar to a tv show
    ///
    /// This is different from the recommendation system as it looks at genres and keywords.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the show to get similar shows for
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
    /// // get shows that our similar to our show
    /// let similar = tmdb.tv.similar(39373).exec().await;;
    /// # assert!(similar.is_ok())
    /// # }
    /// ```
    pub fn similar(&self, id: i64) -> Cursor<Show> {
        // build the url to query
        let url = format!("{}/3/tv/{}/similar", &self.host, id);
        // build our cursor
        Cursor::new(url, &self.token)
    }

    /// Builds a cursor for shows that are popular
    ///
    /// This lists refreshes daily.
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
    /// // get shows that are popular in the us
    /// let popular = tmdb.tv.popular()
    ///   // you can optionally set a region to filter on
    ///   .param("region", "USA")
    ///   .exec()
    ///   .await;
    /// # assert!(popular.is_ok())
    /// # }
    /// ```
    pub fn popular(&self) -> Cursor<Show> {
        // build the url to query
        let url = format!("{}/3/tv/popular", &self.host);
        Cursor::new(url, &self.token)
    }
}
