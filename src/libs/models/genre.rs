use serde::{Deserialize, Serialize};

/// A genre for media
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Genre {
    /// The id for this genre
    pub id: i64,
    /// The name of this genre
    pub name: String,
}
