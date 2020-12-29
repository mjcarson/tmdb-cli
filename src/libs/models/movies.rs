use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use super::{Genre, Language, ProductionCompany, ProductionCountry};

/// Details from searching for [`Movie`] by name
#[derive(Serialize, Deserialize, Debug)]
pub struct Movie {
    /// The path to the poster for this movie
    pub poster_path: Option<String>,
    /// Whether this is an adult movie or not
    pub adult: bool,
    /// A brief overview of this Movie
    pub overview: String,
    /// When this movie was released
    pub release_date: NaiveDate,
    /// The genre IDs for this movie
    #[serde(default)]
    pub genre_ids: Vec<i64>,
    /// The id for this movie
    pub id: i64,
    /// The original title for this movie
    pub original_title: String,
    /// The original language for this movie
    pub original_lnguage: Option<String>,
    /// The title of this movie
    pub title: String,
    /// The path to the backdrop for this movie
    pub backdrop_path: Option<String>,
    /// The popularity of this movie
    pub popularity: f64,
    /// The number of votes for this movie
    pub vote_count: u64,
    /// Whether this is not a movie but another type of longer video
    pub video: bool,
    /// The average vote for this movie
    pub vote_average: f64,
}

/// A cursor from a movie search
#[derive(Serialize, Deserialize, Debug)]
pub struct MovieList {
    /// What page in the search this cursor is for
    pub page: u64,
    /// The movies found
    pub results: Vec<Movie>,
    /// The total number of results found with this search
    pub total_results: u64,
    /// The total number of pages found with this search
    pub total_pages: u64,
}

/// Details on a Movies
#[derive(Serialize, Deserialize, Debug)]
pub struct MovieDetails {
    /// Whether this movie is an adult movie or not
    pub adult: bool,
    /// The path the backdrop for this movie can be found at
    pub backdrop_path: Option<String>,
    /// Whether this movie belongs to a collection or not
    pub belongs_to_collection: Option<bool>,
    /// The budget for this movie
    pub budget: i64,
    // The list of genres this movie is apart of
    pub genres: Vec<Genre>,
    /// The homepage for this Movie
    pub homepage: Option<String>,
    /// The id for this movie
    pub id: i64,
    /// The imdb ID for this movie
    pub imdb_id: Option<String>,
    /// The Original language this was in
    pub original_language: String,
    /// The original title fo this movie
    pub original_title: String,
    /// An overview for this movie
    pub overview: Option<String>,
    /// The popularity of this movie
    pub popularity: f64,
    /// The path to the poster for this movie
    pub poster_path: Option<String>,
    /// The production companies involved in making this movie
    pub production_companies: Vec<ProductionCompany>,
    /// The countries this movie was produced in
    pub production_countries: Vec<ProductionCountry>,
    /// When this movie was released
    pub release_date: NaiveDate,
    /// How much this movie made in revenue
    pub revenue: i64,
    /// The total runtime of this movie in minutes
    pub runtime: Option<i64>,
    /// The languages spoken in this movie
    pub spoken_languages: Vec<Language>,
    /// The current status of this movie
    pub status: String,
    /// The tag line or slogan for this movie
    pub tagline: Option<String>,
    /// The title of this movie
    pub title: String,
    /// Whether this is not a movie but another type of longer video
    pub video: bool,
    /// The average vote for this movie
    pub vote_average: f64,
    /// The number of votes for this movie
    pub vote_count: i64,
}
