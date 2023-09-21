use axum::{extract::State, Json, Router};

use crate::{
    controller::movies_controller::MovieController,
    error::Result,
    model::movies_model::{Movie, MovieForCreate},
};

pub fn routes() -> Router {
    Router::new()
}

async fn create_movie(
    State(mc): State<MovieController>,
    Json(movie_for_create): Json<MovieForCreate>,
) -> Result<Json<Movie>> {
    println!("->> {:<12} - CREATE_MOVIE", "HANDLER");

    todo!("Create a movie");

    // Ok(Json(movie))
}
