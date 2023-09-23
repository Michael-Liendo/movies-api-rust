use axum::{extract::State, routing::post, Json, Router};

use crate::{
    controller::movies_controller::MovieController,
    error::Result,
    model::movies_model::{Movie, MovieForCreate},
};

pub fn routes(mc: MovieController) -> Router {
    Router::new()
        .route("/movies", post(create_movie).get(list_movies))
        .with_state(mc)
}

async fn create_movie(
    State(mc): State<MovieController>,
    Json(movie_for_create): Json<MovieForCreate>,
) -> Result<Json<Movie>> {
    println!("->> {:<12} - CREATE_MOVIE", "HANDLER");

    let movie = mc.create_movie(movie_for_create).await?;

    Ok(Json(movie))
}

async fn list_movies(State(mc): State<MovieController>) -> Result<Json<Vec<Movie>>> {
    println!("->> {:<12} - LIST_MOVIES", "HANDLER");

    let movies = mc.list_movies().await?;

    Ok(Json(movies))
}
