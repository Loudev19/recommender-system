pub mod distances;
pub mod movie_api;
pub mod book_api;
pub mod small_movielens_api;

fn main() {
    /*movie_api::distance_by_name(String::from("Heather"), String::from("Patrick C"), distances::Distance::Manhattan);
    movie_api::distance_by_id(String::from("1"), String::from("2"), distances::Distance::Manhattan);
    book_api::distance_by_id(String::from("26182"), String::from("37400"), distances::Distance::Manhattan);
    small_movielens_api::distance_by_id(String::from("125"), String::from("567"), distances::Distance::Manhattan);

    println!("\n");

    movie_api::distance_by_name(String::from("Heather"), String::from("Patrick C"), distances::Distance::Euclidean);
    movie_api::distance_by_id(String::from("1"), String::from("2"), distances::Distance::Euclidean);
    book_api::distance_by_id(String::from("26182"), String::from("37400"), distances::Distance::Euclidean);
    small_movielens_api::distance_by_id(String::from("125"), String::from("567"), distances::Distance::Euclidean);

    println!("\n");

    movie_api::distance_by_name(String::from("Heather"), String::from("Patrick C"), distances::Distance::Minkowski(3));
    movie_api::distance_by_id(String::from("1"), String::from("2"), distances::Distance::Minkowski(3));
    book_api::distance_by_id(String::from("26182"), String::from("37400"), distances::Distance::Minkowski(3));
    small_movielens_api::distance_by_id(String::from("125"), String::from("567"), distances::Distance::Minkowski(3));

    println!("\n");

    movie_api::distance_by_name(String::from("Heather"), String::from("Patrick C"), distances::Distance::Pearson);
    movie_api::distance_by_id(String::from("1"), String::from("2"), distances::Distance::Pearson);
    book_api::distance_by_id(String::from("26182"), String::from("37400"), distances::Distance::Pearson);
    small_movielens_api::distance_by_id(String::from("125"), String::from("567"), distances::Distance::Pearson);

    println!("\n");

    movie_api::distance_by_name(String::from("Heather"), String::from("Patrick C"), distances::Distance::Cosine);
    movie_api::distance_by_id(String::from("1"), String::from("2"), distances::Distance::Cosine);
    book_api::distance_by_id(String::from("26182"), String::from("37400"), distances::Distance::Cosine);
    small_movielens_api::distance_by_id(String::from("125"), String::from("567"), distances::Distance::Cosine);

    println!("\n");

    movie_api::distance_by_name(String::from("Heather"), String::from("Patrick C"), distances::Distance::JaccardDist);
    movie_api::distance_by_id(String::from("1"), String::from("2"), distances::Distance::JaccardDist);
    book_api::distance_by_id(String::from("26182"), String::from("37400"), distances::Distance::JaccardDist);
    small_movielens_api::distance_by_id(String::from("125"), String::from("567"), distances::Distance::JaccardDist);

    println!("\n");

    movie_api::distance_by_name(String::from("Heather"), String::from("Patrick C"), distances::Distance::JaccardSim);
    movie_api::distance_by_id(String::from("1"), String::from("2"), distances::Distance::JaccardSim);
    book_api::distance_by_id(String::from("26182"), String::from("37400"), distances::Distance::JaccardSim);
    small_movielens_api::distance_by_id(String::from("125"), String::from("567"), distances::Distance::JaccardSim);

    println!("\n");

    movie_api::knn_by_id(3, String::from("1"), distances::Distance::Manhattan);
    movie_api::knn_by_id(3, String::from("1"), distances::Distance::Pearson);
    movie_api::knn_by_id(3, String::from("1"), distances::Distance::Cosine);

    movie_api::knn_by_name(3, String::from("Patrick C"), distances::Distance::Manhattan);
    movie_api::knn_by_name(3, String::from("Patrick C"), distances::Distance::Pearson);
    movie_api::knn_by_name(3, String::from("Patrick C"), distances::Distance::Cosine);

    println!("books");

    book_api::knn_by_id(3, String::from("26182"), distances::Distance::Manhattan);
    book_api::knn_by_id(3, String::from("26182"), distances::Distance::Pearson);
    book_api::knn_by_id(3, String::from("26182"), distances::Distance::Cosine);

    println!("movielens");

    small_movielens_api::knn_by_id(3, String::from("567"), distances::Distance::Manhattan);
    small_movielens_api::knn_by_id(3, String::from("567"), distances::Distance::Pearson);
    small_movielens_api::knn_by_id(3, String::from("567"), distances::Distance::Cosine);

    println!();*/

    movie_api::knn_prediction(10, None, None, Some(String::from("Patrick C")), Some(String::from("Gladiator")), distances::Distance::Pearson);
    small_movielens_api::knn_prediction(10, String::from("567"), Some(String::from("1214")), None, distances::Distance::Pearson);
    book_api::knn_prediction(10, String::from("26182"), Some(String::from("0060987529")), None, distances::Distance::Pearson);
}
