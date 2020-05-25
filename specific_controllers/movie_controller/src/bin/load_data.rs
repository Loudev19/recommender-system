use movie_controller::models::{FoundUser, FoundScore, FoundMovie};
use movie_controller::models::{CreateUser, CreateScore, CreateMovie};
use movie_controller::schema::{users, movies, scores};

use diesel::prelude::*;
use diesel::pg::PgConnection; //Mantiene la conexion

use csv;

fn main() {
    let pg_conn = PgConnection::establish("postgres://maria:@localhost/movie_database")
        .expect("Error connection");
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
        .expect("msg")
}
