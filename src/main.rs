pub mod distances_user;
pub mod distances_item;
pub mod movie_api;
pub mod book_api;
pub mod small_movielens_api;

//use std::collections::HashMap;

fn main() {
/*
    let mut scores = HashMap::new();
    let mut item1 = HashMap::new();
    item1.insert(2, 3.0);
    item1.insert(3, 5.0);
    item1.insert(4, 4.0);
    item1.insert(5, 1.0);
    scores.insert(1, item1);
    let mut item2 = HashMap::new();
    item2.insert(2, 3.0);
    item2.insert(3, 4.0);
    item2.insert(4, 4.0);
    item2.insert(5, 1.0);
    scores.insert(2, item2);
    let mut item3 = HashMap::new();
    item3.insert(1, 4.0);
    item3.insert(2, 3.0);
    item3.insert(4, 3.0);
    item3.insert(5, 1.0);
    scores.insert(3, item3);
    let mut item4 = HashMap::new();
    item4.insert(1, 4.0);
    item4.insert(2, 4.0);
    item4.insert(3, 4.0);
    item4.insert(4, 3.0);
    item4.insert(5, 1.0);
    scores.insert(4, item4);
    let mut item5 = HashMap::new();
    item5.insert(1, 5.0);
    item5.insert(2, 4.0);
    item5.insert(3, 5.0);
    item5.insert(5, 3.0);
    scores.insert(5, item5);
    
    let result = distances_item::acosine_similarity_between(1, 2, &scores);
    println!("{}", result);*/

    //movie_api::distance_item_by_name("Alien", "Avatar");
    /*
    let (order, matrix) = movie_api::matrix_adjusted_cosine();
    movie_api::get_from_matrix("Pulp Fiction", "Toy Story", order.clone(), matrix.clone());
    movie_api::get_from_matrix("Star Wars", "Jaws", order.clone(), matrix.clone());
    movie_api::get_from_matrix("Alien", "Avatar", order.clone(), matrix.clone());
    movie_api::get_from_matrix("Braveheart", "The Happening", order.clone(), matrix.clone());*/

    //small_movielens_api::distance_item_by_name("Toy Story (1995)", "Jumanji (1995)");
    let (order, matrix) = small_movielens_api::matrix_adjusted_cosine();
    small_movielens_api::get_from_matrix("Sabrina (1995)", "Casino (1995)", order.clone(), matrix.clone());
    small_movielens_api::get_from_matrix("Dangerous Minds (1995)", "Friday (1995)", order.clone(), matrix.clone());
    small_movielens_api::get_from_matrix("Iron Will (1994)", "Spider-Man (2002)", order.clone(), matrix.clone());
    small_movielens_api::get_from_matrix("Micmacs (Micmacs à tire-larigot) (2009)", "Room, The (2003)", order.clone(), matrix.clone());
    //no usuarios en comun en Micmacs y Room
}
