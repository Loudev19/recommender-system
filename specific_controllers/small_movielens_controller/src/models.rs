use std::collections::HashMap;

use generic_controller::{User, Item};

use crate::schema::*;

#[derive(diesel::Queryable)]
pub struct FoundUser {
    pub id: i32,
}

#[derive(diesel::Queryable)]
pub struct FoundMovie {
    pub id: i32,
    pub title: String,
    pub genres: String
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
    pub id: i32,
}

#[derive(Insertable)]
#[table_name="movies"]
pub struct CreateMovie {
    pub id: i32,
    pub title: String,
    pub genres: String
}

#[derive(Insertable)]
#[table_name="scores"]
pub struct CreateScore {
    pub userid: i32,
    pub movieid: i32,
    pub score: f64,
}

#[derive(Debug)]
pub struct SMUser {
    pub id: i32,
    pub score: HashMap<i32, f64>
}

#[derive(Debug)]
pub struct SMItem {
    pub id: i32,
    pub name: String,
    pub data: HashMap<String, String>,
}

impl User<SMItem> for SMUser {
    type Id = i32;

    fn id(&self) -> i32{
        self.id
    }

	fn name(&self) -> String{
        String::from("")
    }

	fn data(&self) -> HashMap<String, String>{
        HashMap::new()
    }

	fn scores(&self) -> HashMap<i32, f64>{
        self.score.clone()
    }
}

impl Item for SMItem {
    type Id = i32;

    fn id(&self) -> i32{
        self.id
    }

	fn name(&self) -> String{
        self.name.clone()
    }

	fn data(&self) -> HashMap<String, String>{
        self.data.clone()
    } //extra
}

impl SMItem {
    pub fn new(id:i32, name: String, genres: String) -> SMItem{
        let mut data = HashMap::new();
        data.insert(String::from("Genres"), genres);
        SMItem{id: id, name: name, data: data}
    }
}