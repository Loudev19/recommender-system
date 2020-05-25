use std::collections::HashMap;

use generic_controller::User;
use generic_controller::Item;

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