use small_movielens_controller::models::{FoundUser, FoundScore, FoundMovie};
use small_movielens_controller::models::{CreateUser, CreateScore, CreateMovie};
use small_movielens_controller::schema::{users, movies, scores};

use diesel::prelude::*;
use diesel::pg::PgConnection; //Mantiene la conexion

use generic_controller::GenericController;
use small_movielens_controller::sml_controller::SmallMovielensController;

use std::collections::HashSet;

use csv;

fn main() {
    let connector = PgConnection::establish("postgres://maria:@localhost/small_movielens_database")
        .expect("Failed connection to database");
    
    let mut users = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_path("./ml-latest-small/ratings.csv")
        .expect("Error to load data scores from csv file");

    let mut vector_users = Vec::new();
    let mut ids_users = HashSet::new();
    let mut vector_scores = Vec::new();
    let controller = SmallMovielensController::new();
    println!("Start reading scores and users csv");

    for user in users.records() {
        if let Ok(user_ok) = user {
            let userid = user_ok[0].parse().expect("Error to parse userid in scores table");

            let movieid = user_ok[1].parse().expect("Error to parse movieid in scores table");
            let score = user_ok[2].parse().expect("Error to parse score in scores table");

            if !ids_users.contains(&userid) {
                ids_users.insert(userid);
                vector_users.push(CreateUser{id: userid});
            }


            if controller.get_user_by_id(userid).is_empty() {
                continue;
            }
            if controller.get_item_by_id(movieid).is_empty() {
                continue;
            }
            vector_scores.push(CreateScore{userid: userid, movieid: movieid, score: score});

        }
    }

    let mut movies = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_path("./ml-latest-small/movies.csv")
        .expect("Error to load data movies from csv file");

    let mut vector_movies = Vec::new();
    println!("Start reading movies csv");

    for movie in movies.records() {
        if let Ok(movie_ok) = movie {
            let id = movie_ok[0].parse().expect("Error to parse movieid in scores table");
            let title = &movie_ok[1].to_string();
            let genres = &movie_ok[2].to_string();

            vector_movies.push(CreateMovie{id: id, title: title.clone(), genres: genres.clone()});

        }
    }

    println!("Start insert users");

    for chunk in vector_users.chunks(10000) {
 //       diesel::insert_into(users::table).values(chunk).execute(&connector).expect("Failed insertion of users chunk");
    }

    println!("Start inserting movies");

    for chunk in vector_movies.chunks(10000) {
   //     diesel::insert_into(movies::table).values(chunk).execute(&connector).expect("Error inserting books");
    }

    println!("Start inserting scores {}", vector_scores.len());

    for chunk in vector_scores.chunks(10000) {
//        diesel::insert_into(scores::table).values(chunk).execute(&connector).expect("Error inserting books");
    }
}