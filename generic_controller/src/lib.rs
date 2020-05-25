use std::collections::HashMap;

pub trait GenericController<U: User<I>, I: Item> {
	fn new() -> Self;

	fn get_user_by_name(&self, name: &str) -> Vec<U>;
	fn get_user_by_id(&self, uid: U::Id) -> Vec<U>;
	fn get_item_by_name(&self, name: &str) -> Vec<I>;
	fn get_item_by_id(&self, uid: I::Id) -> Vec<I>;
	fn get_all_users(&self) -> Vec<U>;
}

pub trait User<I: Item> {
	type Id;

	fn id(&self) -> Self::Id;
	fn name(&self) -> String;
	fn data(&self) -> HashMap<String, String>; //extra
	fn scores(&self) -> HashMap<I::Id, f64>; //pelicula id, score
}

pub trait Item {
	type Id;

	fn id(&self) -> Self::Id;
	fn name(&self) -> String;
	fn data(&self) -> HashMap<String, String>; //extra
}
