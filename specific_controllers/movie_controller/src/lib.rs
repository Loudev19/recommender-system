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
    }
}
