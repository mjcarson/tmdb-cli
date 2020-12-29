use serde::{Deserialize, Serialize};

/// A review for a movie or TV show
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReviewAuthor {
    /// The name of this reviewer
    pub name: String,
    /// The username for this reviewer
    pub username: String,
    /// The path to this users avatar
    pub avatar_path: Option<String>,
    /// The rating this user gave the movie
    pub rating: Option<f64>,
}

/// A review for a movie or TV show
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Review {
    /// The author of this review
    pub author: String,
    /// Details about this author
    pub author_details: ReviewAuthor,
    /// The contents of the review
    pub content: String,
}
