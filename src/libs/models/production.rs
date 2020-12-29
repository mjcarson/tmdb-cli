use serde::{Deserialize, Serialize};

/// A Production company for a movie or TV show
#[derive(Deserialize, Serialize, Debug)]
pub struct ProductionCompany {
    /// The name of this production company
    pub name: String,
    /// The id for this production company
    pub id: i64,
    /// The path to the logo for this company
    pub logo_path: Option<String>,
    /// The country of origin for this production company
    pub origin_country: String,
}

/// A country where production of a movie or TV show took place at
#[derive(Deserialize, Serialize, Debug)]
pub struct ProductionCountry {
    /// The ISO 3166-1 code for the name of this country
    pub iso_3166_1: String,
    /// The name of this country
    pub name: String,
}
