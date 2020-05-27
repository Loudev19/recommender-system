use crate::distances;
use crate::distances::Distance;

use generic_controller::{GenericController, User};
use book_controller::b_controller::BookController;

pub fn distance_by_id(idx:String, idy:String, distance_type: Distance) {
    let controller = BookController::new();

    let usersx = controller.get_user_by_id(idx.parse().expect("First id user in book manhattan not parsed"));
    let usersy = controller.get_user_by_id(idy.parse().expect("Second id user in book manhattan not parsed"));

    if usersx.is_empty() {
        println!("No user with id {} found!", idx);
        return;
    }
    if usersy.is_empty() {
        println!("No user with id {} found!", idy);
        return;
    }

    let userx = &usersx[0];
    let usery = &usersy[0];

    let distance = match distance_type {
        Distance::Manhattan => {
            distances::manhattan_distance_between(&userx.scores(), &usery.scores())
        }
        Distance::Euclidean => {
            distances::euclidean_distance_between(&userx.scores(), &usery.scores())
        }
        Distance::Minkowski(grade) => {
            distances::minkowski_distance_between(&userx.scores(), &usery.scores(), grade)
        }
        Distance::Pearson => {
            distances::pearson_correlation_between(&userx.scores(), &usery.scores())
        }
        Distance::Cosine => {
            distances::cosine_similarity_between(&userx.scores(), &usery.scores())
        }
        Distance::JaccardDist => {
            distances::jaccard_distance_between(&userx.scores(), &usery.scores())
        }
        Distance::JaccardSim => {
            distances::jaccard_similarity_between(&userx.scores(), &usery.scores())
        }
    };

    println!("{dist_name:?} Distance between id({x}) and id({y}) is: {distance}", x = userx.id, y = usery.id, distance = distance, dist_name = distance_type);
}

pub fn knn_by_id(k: i32, idx: String, distance_type: Distance) {
    let controller = BookController::new();

    let usersx = controller.get_user_by_id(idx.parse().expect("First id user in book knn not parsed"));

    if usersx.is_empty() {
        println!("No user with id {} found!", idx);
        return;
    }

    let userx = &usersx[0];

    let scores = controller.get_all_scores(); 

    let neighbors = distances::knn(k, userx.id, &scores, distance_type.clone());

    println!("k({}) nearest neighbors for ({})", k, userx.id);
    for it in neighbors {
        let other = &controller.get_user_by_id(it.id)[0];
        println!("with ({}) distance {:?} is {}", other.id, distance_type.clone(), it.distance);
    }
    println!("\n");
}