use std::collections::HashMap;

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

    println!("Book database, {:?} distance between:", distance_type);
    println!("User id({x}) and user id({y}) is: {distance}\n", x = userx.id, y = usery.id, distance = distance);
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

    println!("Book database, K({}) nearest neighbors for ({}) with {:?}", k, userx.id, distance_type.clone());
    for it in neighbors {
        let other = &controller.get_user_by_id(it.id)[0];
        println!("With ({}) distance is {}", other.id, it.distance);
    }
    println!();
}

pub fn knn_prediction(k: i32, idx: String, idy: Option<String>, itemy: Option<String>, distance_type: Distance) {
    let controller = BookController::new();
    
    let result_user = controller.get_user_by_id(idx.parse().expect("Error parsing useris"));

    if result_user.is_empty() {
        println!("No user in database movie");
        return;
    }

    let mut result_item = Vec::new();

    if let Some(itemname) = itemy {
        result_item = controller.get_item_by_name(&itemname);
    } else {
        if let Some(itemid) = idy {
            result_item = controller.get_item_by_id(itemid.parse().expect("Error parsing itemid"));
        } else {
            println!("No item name nor item id");
            return;
        }
    }

    if result_item.is_empty() {
        println!("No item in database movie");
        return;
    }

    let scores = controller.get_all_scores();

    for user in &result_user {
        let neighbors = distances::knn(k, user.id, &scores, distance_type.clone());
        let mut musers_neighbors= Vec::new();
        for n in &neighbors {
            musers_neighbors.push(controller.get_user_by_id(n.id)[0].clone());
        }
        
        let mut pearson_results = Vec::new();
        
        for muser in &musers_neighbors {
            let pearson_comp = distances::pearson_correlation_between(&user.scores(), &muser.scores());
            pearson_results.push(pearson_comp);
        }

        for item in &result_item {
            let mut total_influence = 0.0;
            let mut prediction = 0.0;
            for i in 0..neighbors.len() {
                if pearson_results[i] == distances::DISABLE {continue;}
                if musers_neighbors[i].scores().contains_key(&item.id) {
                    //println!("{} {}", musers_neighbors[i].id, pearson_results[i]);
                    let score_item = musers_neighbors[i].scores().get(&item.id).unwrap().clone();
                    total_influence += pearson_results[i];
                    prediction += score_item * pearson_results[i];
                }
            }

            prediction /= total_influence;
            println!("Book database, prediction for user ({})", user.id);
            println!("With item {}({}) score is: {}\n", item.title, item.id, prediction);
        }
    }
}