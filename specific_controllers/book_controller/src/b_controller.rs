use diesel::prelude::*;
use diesel::pg::PgConnection; //Matiene la conexion

use generic_controller::GenericController;

use crate::models::{BUser, BItem};
use crate::models::{FoundUser, FoundBook, FoundScore};

use crate::schema::{users, books, scores};

use std::collections::HashMap;

pub struct BookController {
    pub conn:PgConnection
}

impl GenericController<BUser, BItem> for BookController {
    fn new() -> Self{
        let pg_conn = PgConnection::establish("postgres://maria:@localhost/book_database")
            .expect("Error establishing connection");
        
        BookController{conn: pg_conn}
    }

	fn get_user_by_name(&self, name: &str) -> Vec<BUser>{
        Vec::new()
    }

	fn get_user_by_id(&self, uid: i32) -> Vec<BUser>{
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
            hash_scores.insert(score.bookid.clone(), score.score);
        }

        let buser = BUser::new(uid, result[0].city.clone(), result[0].age, hash_scores);
        vec![buser]
    }

    fn get_item_by_name(&self, name: &str) -> Vec<BItem>{
        let results = books::table.filter(books::title.eq(name))
            .load::<FoundBook>(&self.conn)
            .expect("Error to get movie by name");
        
        let mut found_book = Vec::new();

        for fbook in &results {
            found_book.push(BItem{id: fbook.id.clone(), title: fbook.title.clone()});
        }

        found_book
    }

	fn get_item_by_id(&self, uid: String) -> Vec<BItem>{
        let results = books::table.filter(books::id.eq(uid))
            .load::<FoundBook>(&self.conn)
            .expect("Error to get book by id");
        
        if results.is_empty() {
            return Vec::new();
        }

        vec![BItem{id: results[0].id.clone(), title: results[0].title.clone()}]
    }

	fn get_all_users(&self) -> Vec<BUser>{
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
                hash_scores.insert(score.bookid.clone(), score.score);
            }

            let buser = BUser::new(fuser.id, fuser.city, fuser.age, hash_scores);
            found_users.push(buser);
        }

        found_users
    }

    fn get_all_scores(&self) -> HashMap<i32, HashMap<String, f64> > {
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
            scores.insert(fscore.bookid.clone(), fscore.score);
        }

        users
    }
}