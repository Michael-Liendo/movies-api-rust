use std::sync::{Arc, Mutex};

use crate::{
    error::Result,
    model::movies_model::{Movie, MovieForCreate},
};

#[derive(Clone)]
pub struct MovieController {
    movies_store: Arc<Mutex<Vec<Option<Movie>>>>,
}

impl MovieController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            movies_store: Arc::default(),
        })
    }

    pub async fn create_movie(&self, movie_for_create: MovieForCreate) -> Result<Movie> {
        let mut store = self.movies_store.lock().unwrap();

        let movie = Movie {
            id: store.len() as i32 + 1,
            title: movie_for_create.title,
            year: movie_for_create.year,
            length: movie_for_create.length,
            genre: movie_for_create.genre,
            studio: movie_for_create.studio,
            rating: movie_for_create.rating,
            format: movie_for_create.format,
            owner: movie_for_create.owner,
        };

        store.push(Some(movie.clone()));

        Ok(movie)
    }

    pub async fn list_movies(&self) -> Result<Vec<Movie>> {
        let store = self.movies_store.lock().unwrap();

        let movies = store
            .iter()
            .filter_map(|movie| movie.clone())
            .collect::<Vec<_>>();

        Ok(movies)
    }

    pub async fn get_movie(&self, id: i32) -> Result<Option<Movie>> {
        let store = self.movies_store.lock().unwrap();

        let movie = store
            .iter()
            .filter_map(|movie| movie.clone())
            .find(|movie| movie.id == id);

        Ok(movie)
    }

    pub async fn edit_movie(&self, id: i32, movie_for_edit: MovieForCreate) -> Result<Movie> {
        let mut store = self.movies_store.lock().unwrap();

        let movie = store
            .iter_mut()
            .find(|movie| movie.as_ref().map_or(false, |m| m.id == id));

        let movie = movie.unwrap();

        if let Some(movie) = movie {
            movie.title = movie_for_edit.title;
            movie.year = movie_for_edit.year;
            movie.length = movie_for_edit.length;
            movie.genre = movie_for_edit.genre;
            movie.studio = movie_for_edit.studio;
            movie.rating = movie_for_edit.rating;
            movie.format = movie_for_edit.format;
            movie.owner = movie_for_edit.owner;
        }

        Ok(movie.clone().unwrap())
    }

    pub async fn delete_movie(&self, id: i32) -> Result<Option<Movie>> {
        let mut store = self.movies_store.lock().unwrap();

        let movie = store.get_mut(id as usize).and_then(|m| m.take());

        Ok(movie)
    }
}
