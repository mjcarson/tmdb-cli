use chrono::prelude::*;
use serde::{Serialize, Deserialize};

use super::{Genre, ProductionCompany, ProductionCountry, Language};

/// Details from searching for [`Show`] by name
#[derive(Serialize, Deserialize, Debug)]
pub struct Show {
    /// The path to the poster for this show
    pub poster_path: Option<String>,
    /// The popularity of this show
    pub popularity: f64,
    /// The id for this show
    pub id: i64,
    /// The path to the backdrop for this show
    pub backdrop_path: Option<String>,
    /// The average vote for this show
    pub vote_average: f64,
    /// A brief overview of this Movie
    pub overview: String,
    /// When this show was first aired
    pub first_air_date: NaiveDate,
    /// The countries this show comes from
    pub origin_country: Vec<String>,
    /// The genre IDs for this show
    #[serde(default)]
    pub genre_ids: Vec<i64>,
    /// The original language for this show
    pub original_lnguage: Option<String>,
    /// The number of votes for this show
    pub vote_count: u64,
    /// The name of this show
    pub name: String,
}

/// The creator of TV show
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TvCreator {
    /// The ID for this creator
    pub id: i64,
    /// An ID for this specific creator in order to differentiate people with multiple credits
    pub credit_id: String,
    /// The name of the person who created this show
    pub name: String,
    /// The gender of this person represented as a int (0 = M, 1 = F, 2 = N/A)
    pub gender: Option<i64>,
    /// The path to retrieve this crew members profile at
    pub profile_path: Option<String>,
}

/// An episode of a TV show
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Episode {
    /// The date this episode aired
    pub air_date: NaiveDate,
    /// The number for this episode
    pub episode_number: u64,
    /// The id for this episode
    pub id: i64,
    /// The name of this episode
    pub name: String,
    /// A synopsis of this episode
    pub overview: String,
    /// The production code for identifying this episode
    pub production_code: String,
    /// What season this episode is from
    pub season_number: i64,
    /// The path the still image for this episode can be found at
    pub still_path: Option<String>,
    /// The average vote for this episode
    pub vote_average: f64,
    /// The number of votes for this episode
    pub vote_count: i64,
}

/// A TV network
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Network {
    /// The name of this network
    pub name: String,
    /// The id for this network
    pub id: i64,
    /// The path this networks logo can be found at
    pub logo_path: Option<String>,
    /// The country this network originates from
    pub origin_country: String,
}

/// A season for a TV show
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Season {
    /// The date this season was first aired
    pub air_date: NaiveDate,
    /// The number of episodes in this season
    pub episode_count: u64,
    /// The id for this season
    pub id: i64,
    /// The name of this season
    pub name: String,
    /// A synopsis of this season
    pub overview: String,
    /// The path this seasons poster can be found at
    pub poster_path: Option<String>,
    /// The number this season is
    pub season_number: i64,
}

/// Details about a TV show
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ShowDetails {
    /// The path this shows backdrop image can be found at
    pub backdrop_path: Option<String>,
    /// The creators of this TV show
    pub created_by: Vec<TvCreator>,
    /// The episode runtimes for this Show,
    pub episode_run_time: Vec<i64>,
    /// When this show was first aired
    pub first_air_date: NaiveDate,
    /// The genres for this show
    pub genres: Vec<Genre>,
    /// The homepage for this show
    pub homepage: String,
    /// The unique ID for this show
    pub id: i64,
    /// Whether this show is still in production,
    pub in_production: bool,
    /// The languages in this show
    pub languages: Vec<String>,
    /// The date the last episode aired
    pub last_air_date: NaiveDate,
    /// The last episode to be aired
    pub last_episode_to_air: Episode,
    /// The name of this show
    pub name: String,
    /// The date the next episode airs
    pub next_episode_to_air: Option<NaiveDate>,
    /// The networks involved with this show
    pub networks: Vec<Network>,
    /// The number of episodes in this show
    pub number_of_episodes: u64,
    /// The number of seasons in this show
    pub number_of_seasons: u64,
    /// The countries this show comes from
    pub origin_country: Vec<String>,
    /// The original language of this show
    pub original_language: String,
    /// The original name of this show
    pub original_name: String,
    /// A synopsis of this season
    pub overview: String,
    /// The popularity of this show
    pub popularity: f64,
    /// The path to the poster for this show
    pub poster_path: Option<String>,
    /// The production companies involved in making this show
    pub production_companies: Vec<ProductionCompany>,
    /// The countries this show was produced in
    pub production_countries: Vec<ProductionCountry>,
    /// The seasons in this show
    pub seasons: Vec<Season>,
    /// The lanugages spoken in this show
    pub spoken_languages: Vec<Language>,
    /// The current status of this show
    pub status: String,
    /// The tagline or slogan of this show
    pub tagline: String,
    /// What type of show this is
    #[serde(rename = "type")]
    pub _type: String, 
    /// The average vote for this show
    pub vote_average: f64,
    /// The number of votes for this show
    pub vote_count: i64,
}

