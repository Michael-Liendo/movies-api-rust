use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub year: i32,
    pub length: i32,
    pub genre: String,
    pub studio: String,
    pub rating: String,
    pub format: String,
    pub owner: String,
}

#[derive(Deserialize)]
pub struct MovieForCreate {
    pub title: String,
    pub year: i32,
    pub length: i32,
    pub genre: String,
    pub studio: String,
    pub rating: String,
    pub format: String,
    pub owner: String,
}
