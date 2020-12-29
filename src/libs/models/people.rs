use serde::{Deserialize, Serialize};

/// A actor or actress for a movie or TV show
#[derive(Serialize, Deserialize, Debug)]
pub struct Cast {
    /// Whether this is an adult film actor/actress
    pub adult: bool,
    /// The gender of this person represented as a int (0 = M, 1 = F, 2 = N/A)
    pub gender: Option<i64>,
    /// The ID for this cast member
    pub id: i64,
    /// The department this person is known for (acting/lighting/camera...)
    pub known_for_department: String,
    /// The name of this cast member
    pub name: String,
    /// The original name of this cast member
    pub original_name: String,
    /// The popularity of this cast member
    pub popularity: f64,
    /// The path to retrieve this cast members profile at
    pub profile_path: Option<String>,
    /// The legacy unique id for this cast member
    pub cast_id: i64,
    /// The name of the character they play in this particular casting
    pub character: String,
    /// An ID for this specific casting in order to differentiate people with multiple credits in the same cast
    pub credit_id: String,
    /// The order that this casting should be represented in relation to the other casted roles in this cast
    pub order: i64,
}

/// A member of the crew for a movie or TV show
#[derive(Serialize, Deserialize, Debug)]
pub struct Crew {
    /// Whether this is an adult film crew member
    pub adult: bool,
    /// The gender of this person represented as a int (0 = M, 1 = F, 2 = N/A)
    pub gender: Option<i64>,
    /// The ID for this crew member
    pub id: i64,
    /// The department this person is known for (acting/lighting/camera...)
    pub known_for_department: String,
    /// The name of this crew member
    pub name: String,
    /// The original name of this crew member
    pub original_name: String,
    /// The popularity of this crew member
    pub popularity: f64,
    /// The path to retrieve this crew members profile at
    pub profile_path: Option<String>,
    /// An ID for this specific crew member in order to differentiate people with multiple credits
    pub credit_id: String,
    /// The department this crew member worked in
    pub department: String,
    /// The job this crew member held
    pub job: String,
}

/// The cast and crew for a movie or TV show
#[derive(Serialize, Deserialize, Debug)]
pub struct Credits {
    /// The id for this particular cast/crew list
    pub id: i64,
    /// The cast for this movie or TV show
    #[serde(default)]
    pub cast: Vec<Cast>,
    /// The crew for this movie or TV show
    #[serde(default)]
    pub crew: Vec<Crew>,
}
