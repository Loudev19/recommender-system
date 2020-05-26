use book_controller::models::{CreateUser, CreateScore, CreateBook};
use book_controller::schema::{users, books, scores};

use generic_controller::GenericController;
use book_controller::b_controller::BookController;

use diesel::prelude::*;
use diesel::pg::PgConnection; //Mantiene la conexion

use csv;

fn main() {

    let connector = PgConnection::establish("postgres://maria:@localhost/book_database")
        .expect("Failed connection to database");

    let mut users = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_path("./Book_Users.csv")
        .expect("Error to load data from csv file");
    
    
    let mut vector_users = Vec::new();
    println!("Hey! start reading csv users");

    for user in users.records() {
        if let Ok(user_ok) = user {
            let id = user_ok[0].parse().expect("Error");
            let city = &user_ok[1].to_string();
            let age:Option<i32> = if &user_ok[2] == String::from("\\N") {
                None
            } else {
                Some(user_ok[2].parse().expect("Error"))
            };

            vector_users.push(CreateUser{id: id, city: city.clone(), age: age});

        }
    }
    println!("Start insert users");

    for chunk in vector_users.chunks(10000) {
        diesel::insert_into(users::table).values(chunk).execute(&connector).expect("Failed insertion of users chunk");
    }

    let mut books = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_path("./Book_Books.csv")
        .expect("Error reading data from csv file");
    
    let mut vector_books = Vec::new();
    println!("Start reading books csv");

    for book in books.records() {
        if let Ok(book_ok) = book {
            let id = &book_ok[0].to_string();
            let title = &book_ok[1].to_string();
            let author = &book_ok[2].to_string();
            let pubyear = &book_ok[3].to_string();
            let publisher = &book_ok[4].to_string();

            vector_books.push(CreateBook{id: id.clone(), title: title.clone(), author: author.clone(), pubyear: pubyear.clone(), publisher: publisher.clone()});

        }
    }
    println!("Start inserting books");

    for chunk in vector_books.chunks(10000) {
        diesel::insert_into(books::table).values(chunk).execute(&connector).expect("Error inserting books");
    }

    let mut scores = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_path("./Book_Ratings.csv")
        .expect("Error reading data from csv file");
    
    let mut vector_scores = Vec::new();
    let controller = BookController::new();
    println!("Start reading scores csv");

    for s in scores.records() {
        if let Ok(score_ok) = s {
            let userid = score_ok[0].parse().expect("Error");
            let bookid = &score_ok[1].to_string();
            let score = score_ok[2].parse().expect("Error");

            if controller.get_user_by_id(userid).is_empty() { continue; } 
            if controller.get_item_by_id(bookid.clone()).is_empty() { continue; }

            vector_scores.push(CreateScore{userid: userid, bookid: bookid.clone(), score: score});

        }
    }

    println!("Start inserting scores");

    for chunk in vector_scores.chunks(10000) {
        diesel::insert_into(scores::table).values(chunk).execute(&connector).expect("Error inserting books");
    }
}
