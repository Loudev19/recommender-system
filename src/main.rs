use generic_controller::{GenericController, User, Item};
use movie_controller::{m_controller::MovieController, models::{MUser, MItem}};
use book_controller::{b_controller::BookController, models::{BUser, BItem}};
use small_movielens_controller::{sml_controller::SmallMovielensController, models::{SMUser, SMItem}};

pub mod distances;

fn main() {
    println!("Hello, world!");
}
