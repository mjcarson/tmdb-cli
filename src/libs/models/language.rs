use serde::{Serialize, Deserialize};

/// A language that was spoken in a movie or TV show
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Language {
  /// The ISO 369-1 code for this language
  pub iso_639_1: String,
  /// The name of this language
  pub name: String,
}
