use std::collections::HashMap;

use generic_controller::{User, Item};

use crate::schema::*;

#[derive(diesel::Queryable)]
pub struct FoundUser {
    pub id: i32,
    pub uname: String,
}

#[derive(diesel::Queryable)]
pub struct FoundMovie {
    pub id: i32,
    pub title: String,
}

#[derive(diesel::Queryable)]
pub struct FoundScore {
    pub id: i32,
    pub userid: i32,
    pub movieid: i32,
    pub score: f64
}

#[derive(Insertable)]
#[table_name="users"]
pub struct CreateUser {
    pub uname: String,
}

#[derive(Insertable)]
#[table_name="movies"]
pub struct CreateMovie {
    pub title: String,
}

#[derive(Insertable)]
#[table_name="scores"]
pub struct CreateScore {
    pub userid: i32,
    pub movieid: i32,
    pub score: f64,
}

#[derive(Debug)]
pub struct MUser {
    pub id: i32,
    pub name: String,
    pub score: HashMap<i32, f64>
}

#[derive(Debug)]
pub struct MItem {
    pub id: i32,
    pub name: String,
}

impl User for MUser {

    fn id(&self) -> i32{
        self.id
    }

	fn name(&self) -> String{
        self.name.clone()
    }

	fn data(&self) -> HashMap<String, String>{
        HashMap::new()
    }

	fn scores(&self) -> HashMap<i32, f64>{
        self.score.clone()
    }
}

impl Item for MItem {
    fn id(&self) -> i32{
        self.id
    }

	fn name(&self) -> String{
        self.name.clone()
    }

	fn data(&self) -> HashMap<String, String>{
        HashMap::new()
    } //extra
}