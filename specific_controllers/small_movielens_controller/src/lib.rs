#[macro_use]

extern crate diesel;

pub mod schema;

pub mod models;
pub mod sml_controller;

#[cfg(test)]
mod tests {
    use super::sml_controller::SmallMovielensController;
    use generic_controller::{GenericController};

    #[test]
    fn query_user() {
        let manager = SmallMovielensController::new();

        let users = manager.get_user_by_name("Chris");
        
        println!("{:?}\n", users);

        let users = manager.get_user_by_id(2);
        
        println!("{:?}\n", users);

        //Dont do this, it takes a lot of time.. REFACTORING
        //let users = manager.get_all_users();
        
        //println!("{:?}\n", users);

        let books = manager.get_item_by_name("Nixon (1995)");
        
        println!("{:?}\n", books);

        let books = manager.get_item_by_id(10);
        
        println!("{:?}\n", books);

        let ratings = manager.get_all_scores();

        println!("{:?}\n", ratings[&2]);

        let users = manager.get_user_by_id(2);

        println!("{:?}\n", users);
    }
}
