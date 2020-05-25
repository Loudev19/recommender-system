use diesel::prelude::*;
use diesel::pg::PgConnection; //Matiene la conexion

use generic_controller::GenericController;

use crate::models::MUser;

pub struct MovieController {
    pub conn:PgConnection
}

impl GenericController<MUser> for MovieController {
    fn new() -> Self{
        let pg_conn = PgConnection::establish("postgres://maria:@localhost/movie_database")
            .expect("Error establishing connection");
        
        MovieController{conn: pg_conn}
    }

	fn get_user_by_name(&self, name: &str) -> Vec<MUser>{
        todo!()
    }

	fn get_user_by_id(&self, uid: u64) -> Vec<MUser>{
        todo!()
    }

	fn get_all_users(&self) -> Vec<MUser>{
        todo!()
    }
}


