use diesel::prelude::*;
use diesel::pg::PgConnection; //Matiene la conexion

fn main() {
    let pg_conn = PgConnection::establish("postgres://maria:@localhost/movie_database")
        .expect("Error connection");
}
