use std::env;

use super::{movies::Movies, tv::Tv};

/// A TMDB client
pub struct Client {
    /// Movie focused routes handlers
    pub movies: Movies,
    /// TV focused routes handlers
    pub tv: Tv
}

impl Client {
    /// Creates a new client with a token
    ///
    /// # Arguments
    ///
    /// # `token` - The token to authenticate to tmdb with
    ///
    /// # Examples
    ///
    /// ```
    /// use tmdb_cli::Client;
    ///
    /// let tmdb = Client::new("TMDB_TOKEN".into());
    /// ```
    pub fn new(token: String) -> Self {
        // default tmdb host
        let host = "https://api.themoviedb.org";
        let movies = Movies::new(host, &token);
        let tv = Tv::new(host, &token);
        Client { movies, tv }
    }

    /// Creates a new client with a token pulled from the environment
    ///
    /// # Examples
    ///
    /// ```
    /// use tmdb_cli::Client;
    ///
    /// let tmdb = Client::from_env();
    /// ```
    pub fn from_env() -> Self {
        // get the tmdb token from our environment variables
        let token = match env::var("TMDB_TOKEN") {
            Ok(token) => token,
            Err(e) => panic!("Failed to get token from environment with {}", e),
        };
        Self::new(token)
    }
}
