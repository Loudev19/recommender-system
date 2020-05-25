#[macro_use]

extern crate diesel;

pub mod schema;

pub mod models;
pub mod m_controller;

#[cfg(test)]
mod tests {
    use super::m_controller::MovieController;
    use generic_controller::{GenericController};
    #[test]
    fn it_works() {
        let controller = MovieController::new();
        let users = controller.get_user_by_id(10);
        println!("{:?}\n", users);

        let users = controller.get_user_by_name("Chris");
        println!("{:?}\n", users);

        let movies = controller.get_item_by_name("Avatar");
        println!("{:?}\n", movies);

        let movies = controller.get_item_by_id(5);
        println!("{:?}\n", movies);

        let users = controller.get_all_users();
        println!("{:?}\n", users);
    }
}
