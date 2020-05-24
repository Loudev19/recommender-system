use std::collections::HashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub trait GenericController<U: User, I: Item> {
	fn get_user_by_name(&self, name: &str) -> vec<U>;
	fn get_item_by_name(&self, name: &str) -> vec<I>;
	fn get_all_users(&self) -> vec<U>;
}

pub trait Item {
	fn id(&self) -> u64;
	fn name(&self) -> String;
	fn data(&self) -> HashMap<String, String>;
}

pub trait User {
	fn id(&self) -> u64;
	fn name(&self) -> String;
	fn data(&self) -> HashMap<String, String>;
	fn scores(&self) -> HashMap<u64, f64>;
}