use std::collections::HashMap;

use generic_controller::{User, Item};

use crate::schema::*;

#[derive(diesel::Queryable)]
pub struct FoundUser {
    pub id: i32,
    pub city: String,
    pub age: Option<i32>
}

#[derive(diesel::Queryable)]
pub struct FoundBook {
    pub id: String,
    pub title: String,
    pub author: String,
    pub pubyear: String,
    pub publisher: String
}

#[derive(diesel::Queryable)]
pub struct FoundScore {
    pub id: i32,
    pub userid: i32,
    pub bookid: String,
    pub score: f64
}

#[derive(Insertable)]
#[table_name="users"]
pub struct CreateUser {
    pub id: i32,
    pub city: String,
    pub age: Option<i32>,
}

#[derive(Insertable)]
#[table_name="books"]
pub struct CreateBook {
    pub id: String,
    pub title: String,
    pub author: String,
    pub pubyear: String,
    pub publisher: String
}

#[derive(Insertable)]
#[table_name="scores"]
pub struct CreateScore {
    pub userid: i32,
    pub bookid: String,
    pub score: f64,
}

#[derive(Debug)]
pub struct BUser {
    pub id: i32,
    pub data: HashMap<String, String>,
    pub score: HashMap<String, f64>
}
#[derive(Debug)]
pub struct BItem {
    pub id: String,
    pub title: String
}

impl User<BItem> for BUser {
    type Id = i32;

    fn id(&self) -> i32 {
        self.id
    }

	fn name(&self) -> String {
        String::from("")
    }

	fn data(&self) -> HashMap<String, String>{
        self.data.clone()
    }

	fn scores(&self) -> HashMap<String, f64>{
        self.score.clone()
    }

}

impl Item for BItem {
    type Id = String;

    fn id(&self) -> String {
        self.id.clone()
    }

    fn name(&self) -> String {
        self.title.clone()
    }

    fn data(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}

impl BUser {
    pub fn new(id:i32, city: String, age: Option<i32>, scores: HashMap<String, f64>) -> BUser{
        let mut data = HashMap::new();
        data.insert(String::from("City"), city);
        if let Some(age) = age {
            data.insert(String::from("Age"), age.to_string());
        }
        BUser{id: id, data:data, score: scores}
    }
}