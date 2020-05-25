use std::collections::HashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub trait GenericController<U: User> {
	fn connect_to(Url: &str) -> Self;

	fn get_user_by_name(&self, name: &str) -> Vec<U>;
	fn get_user_by_id(&self, uid: u64) -> Vec<U>;
	fn get_all_users(&self) -> Vec<U>;
}

pub trait User {
	fn id(&self) -> u32;
	fn name(&self) -> String;
	fn data(&self) -> HashMap<String, String>;
	fn scores(&self) -> HashMap<u32, f64>; //pelicula id, score
}

pub trait Item {
	fn id(&self) -> i32;
	fn name(&self) -> String;
	fn data(&self) -> HashMap<String, String>; //extra
}
