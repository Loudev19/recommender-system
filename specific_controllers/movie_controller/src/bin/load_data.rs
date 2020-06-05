use movie_controller::models::{FoundUser, FoundScore, FoundMovie};
use movie_controller::models::{CreateUser, CreateScore, CreateMovie};
use movie_controller::schema::{users, movies, scores};

use diesel::prelude::*;
use diesel::pg::PgConnection; //Mantiene la conexion

use csv;

fn main() {
    
    let mut content = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("./Movie_Ratings.csv")
        .expect("Couldn't load from csv file");
    
    let mut matrix = Vec::new();

    for r in content.records() {
        if let Ok(record) = r {
            let mut row = Vec::new();
            for r in record.iter() {
                row.push(String::from(r));
            }
            matrix.push(row);
        }
    }

    let connector = PgConnection::establish("postgres://maria:@localhost/movie_database").expect("Failed connection to database. Maybe the URL?");

    let mut db_users = Vec::new();

    for col in 1..matrix[0].len() {
        db_users.push(create_user(&connector, &matrix[0][col]));
    }

    let mut db_movies = Vec::new();

    for row in 1..matrix.len() {
        db_movies.push(create_movie(&connector, &matrix[row][0]));
    }


    for i in 0..db_users.len() {
        let current_user = &db_users[i];

        for j in 0..db_movies.len() {
            let current_movie = &db_movies[j];
            if matrix[j+1][i+1] != "" {
                create_score(&connector, matrix[j+1][i+1].parse().expect("Failed to parse"), current_user.id, current_movie.id);  
            }
        }
    }
}

fn create_user(conn: &PgConnection, name:&String) -> FoundUser{
    let user =  CreateUser{uname: name.clone()};

    diesel::insert_into(users::table)
        .values(&user)
        .get_result(conn)
        .expect("Failed to create new user")
}

fn create_movie(conn: &PgConnection, name:&String) -> FoundMovie{
    let movie =  CreateMovie{title: name.clone()};

    diesel::insert_into(movies::table)
        .values(&movie)
        .get_result(conn)
        .expect("Failed to create new movie")
}

fn create_score(conn: &PgConnection, xscore:f64, xuserid: i32, xmovieid: i32) -> FoundScore{
    let new_score =  CreateScore{userid: xuserid, movieid: xmovieid, score: xscore};

    diesel::insert_into(scores::table)
        .values(&new_score)
        .get_result(conn)
        .expect("Error creating new score")
}
