use diesel::prelude::*;
use diesel::pg::PgConnection; //Matiene la conexion

use generic_controller::GenericController;

use crate::models::{SMUser, SMItem};
use crate::models::{FoundUser, FoundMovie, FoundScore};

use crate::schema::{users, movies, scores};

use std::collections::HashMap;

pub struct SmallMovielensController {
    pub conn:PgConnection
}

impl GenericController<SMUser, SMItem> for SmallMovielensController {
    fn new() -> Self{
        let pg_conn = PgConnection::establish("postgres://maria:@localhost/small_movielens_database")
            .expect("Error establishing connection");
        
        SmallMovielensController{conn: pg_conn}
    }

	fn get_user_by_name(&self, name: &str) -> Vec<SMUser>{
        Vec::new()
    }

	fn get_user_by_id(&self, uid: i32) -> Vec<SMUser>{
        let result = users::table.filter(users::id.eq(uid))
            .load::<FoundUser>(&self.conn)
            .expect("Error to load user by id");
        
        if result.is_empty() {
            return Vec::new();
        }

        let scores = scores::table.filter(scores::userid.eq(uid))
            .load::<FoundScore>(&self.conn)
            .expect("Error loading scores");
        
        let mut hash_scores = HashMap::new();

        for score in &scores {
            hash_scores.insert(score.movieid.clone(), score.score);
        }

        vec![SMUser{id: uid, score: hash_scores}]
    }

    fn get_item_by_name(&self, name: &str) -> Vec<SMItem>{
        let results = movies::table.filter(movies::title.eq(name))
            .load::<FoundMovie>(&self.conn)
            .expect("Error to get movie by name");
        
        let mut found_movie = Vec::new();

        for fmovie in &results {
            let smitem = SMItem::new(fmovie.id, fmovie.title.clone(), fmovie.genres.clone());
            found_movie.push(smitem);
        }

        found_movie
    }

	fn get_item_by_id(&self, uid: i32) -> Vec<SMItem>{
        let results = movies::table.filter(movies::id.eq(uid))
            .load::<FoundMovie>(&self.conn)
            .expect("Error to get movie by id");
        
        if results.is_empty() {
            return Vec::new();
        }
        vec![SMItem::new(uid, results[0].title.clone(), results[0].genres.clone())]
    }

	fn get_all_users(&self) -> Vec<SMUser>{
        let results = users::table
            .load::<FoundUser>(&self.conn)
            .expect("Error to get all users");
        
        let mut found_users = Vec::new();

        for fuser in results {
            let scores = scores::table.filter(scores::id.eq(fuser.id))
                .load::<FoundScore>(&self.conn)
                .expect("Error to get scores");

            let mut hash_scores = HashMap::new();

            for score in &scores {
                hash_scores.insert(score.movieid, score.score);
            }

            found_users.push(SMUser{id: fuser.id, score: hash_scores});
        }

        found_users
    }

    fn get_all_scores(&self) -> HashMap<i32, HashMap<i32, f64> > {
        let results = scores::table
            .load::<FoundScore>(&self.conn)
            .expect("Failed to get all scores");

        let mut users = HashMap::new();
        for fscore in results {
            if !users.contains_key(&fscore.userid){
                users.insert(fscore.userid, HashMap::new());
            }
            let scores = users.get_mut(&fscore.userid)
                .expect("Failed to get scores");
            scores.insert(fscore.movieid, fscore.score);
        }

        users
    }
}

