use diesel::prelude::*;
use diesel::pg::PgConnection; //Matiene la conexion

use generic_controller::GenericController;

use crate::models::{MUser, MItem};
use crate::models::{CreateUser, CreateMovie, CreateScore};
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
        todo!()
    }

	fn get_user_by_id(&self, uid: i32) -> Vec<MUser>{
        let results = users::table.filter(users::id.eq(uid))
            .limit(1)
            .load::<FoundUser>(&self.conn)
            .expect("Error loading users");
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
        todo!()
    }

	fn get_item_by_id(&self, uid: i32) -> Vec<MItem>{
        todo!()
    }

	fn get_all_users(&self) -> Vec<MUser>{
        todo!()
    }
}


