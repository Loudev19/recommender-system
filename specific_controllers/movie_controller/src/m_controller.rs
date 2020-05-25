use diesel::prelude::*;
use diesel::pg::PgConnection; //Matiene la conexion

use generic_controller::GenericController;

use crate::models::{MUser, MItem};
//use crate::models::{CreateUser, CreateMovie, CreateScore};
use crate::models::{FoundUser, FoundMovie, FoundScore};

use crate::schema::{users, movies, scores};

use std::collections::HashMap;

pub struct MovieController {
    pub conn:PgConnection
}

impl GenericController<MUser, MItem> for MovieController {
    fn new() -> Self{
        let pg_conn = PgConnection::establish("postgres://maria:@localhost/movie_database")
            .expect("Error establishing connection");
        
        MovieController{conn: pg_conn}
    }

	fn get_user_by_name(&self, name: &str) -> Vec<MUser>{
        let results = users::table.filter(users::uname.eq(name))
            .load::<FoundUser>(&self.conn)
            .expect("Error searching users by name");
        if results.is_empty() {
            return Vec::new();
        }

        let mut found_users = Vec::new();

        for fuser in &results {
            let scores = scores::table.filter(scores::userid.eq(fuser.id))
                .load::<FoundScore>(&self.conn)
                .expect("Error searching scores");

            let mut hash_scores = HashMap::new();

            for score in &scores {
                hash_scores.insert(score.movieid, score.score);
            }

            found_users.push(MUser{id: fuser.id, name: fuser.uname.clone(), score: hash_scores});
        }

        found_users
    }

	fn get_user_by_id(&self, uid: i32) -> Vec<MUser>{
        let results = users::table.filter(users::id.eq(uid))
            .limit(1)
            .load::<FoundUser>(&self.conn)
            .expect("Error searching users by id");
        if results.is_empty() {
            return Vec::new();
        }
        
        let scores = scores::table.filter(scores::userid.eq(uid))
            .load::<FoundScore>(&self.conn)
            .expect("Error loading scores");

        let mut hash_scores = HashMap::new();

        for score in &scores {
            hash_scores.insert(score.movieid, score.score);
        }

        vec![MUser{id: uid, name: results[0].uname.clone(), score: hash_scores}]
    }

    fn get_item_by_name(&self, name: &str) -> Vec<MItem>{
        let results = movies::table.filter(movies::title.eq(name))
            .load::<FoundMovie>(&self.conn)
            .expect("Error searching movie by title");
        
        let mut found_movies = Vec::new();

        for movie in &results {
            found_movies.push(MItem{id: movie.id, name: movie.title.clone()});        
        }

        found_movies

    }

	fn get_item_by_id(&self, uid: i32) -> Vec<MItem>{
        let results = movies::table.filter(movies::id.eq(uid))
            .load::<FoundMovie>(&self.conn)
            .expect("Error searching movies by id");
        
        if results.is_empty() {
            return Vec::new();
        }

        vec![MItem{id: uid, name: results[0].title.clone()}]
    }

	fn get_all_users(&self) -> Vec<MUser>{
        let results = users::table
            .load::<FoundUser>(&self.conn)
            .expect("Error searching all users");
        
        let mut found_users = Vec::new();

        for fuser in &results {
            let scores = scores::table.filter(scores::userid.eq(fuser.id))
                .load::<FoundScore>(&self.conn)
                .expect("Error searching scores");

            let mut hash_scores = HashMap::new();

            for score in &scores {
                hash_scores.insert(score.movieid, score.score);
            }

            found_users.push(MUser{id: fuser.id, name: fuser.uname.clone(), score: hash_scores});
        }

        found_users
    }
}


