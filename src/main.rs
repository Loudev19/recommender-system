pub mod distances_user;
pub mod distances_item;
pub mod movie_api;
pub mod book_api;
pub mod small_movielens_api;

use std::collections::HashMap;

fn main() {

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
    println!("{}", result);
}
