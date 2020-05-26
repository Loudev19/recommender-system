//use movie_controller::models::{FoundUser, FoundScore, FoundMovie};
use book_controller::models::{CreateUser, CreateScore, CreateBook};
use book_controller::schema::{users, books, scores};

use diesel::prelude::*;
use diesel::pg::PgConnection; //Mantiene la conexion

use csv;

fn main() {

    let mut content = Vec::new();
    for item in vec!["./Book_Users.csv", "./Book_Books.csv", "./Book_Ratings.csv"] {
        content.push(csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(item)
            .expect("Couldn't load from csv file"));
    }

    let connector = PgConnection::establish("postgres://maria:@localhost/movie_database")
        .expect("Failed connection to database");
}
