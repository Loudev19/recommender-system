#[macro_use]

extern crate diesel;

pub mod schema;

pub mod models;
pub mod sml_controller;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
