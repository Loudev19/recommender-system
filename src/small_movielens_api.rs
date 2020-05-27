use std::collections::HashMap;

use crate::distances;
use crate::distances::Distance;

use generic_controller::{GenericController, User};
use small_movielens_controller::sml_controller::SmallMovielensController;

pub fn distance_by_id(idx:String, idy:String, distance_type:Distance) {
    let controller = SmallMovielensController::new();

    let usersx = controller.get_user_by_id(idx.parse().expect("First id user in movielens manhattan not parsed"));
    let usersy = controller.get_user_by_id(idy.parse().expect("Second id user in movielens manhattan not parsed"));

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
    let controller = SmallMovielensController::new();

    let usersx = controller.get_user_by_id(idx.parse().expect("First id user in small movielens knn not parsed"));

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

pub fn knn_prediction(k: i32, userid: String, itemx: Option<String>, itemname: Option<String>, distance_type: Distance) {
    let controller = SmallMovielensController::new();
    
    let result_user = controller.get_user_by_id(userid.parse().expect("Error parsing useris"));

    if result_user.is_empty() {
        println!("No user in database movie");
        return;
    }

    let mut result_item = Vec::new();

    if let Some(itemname) = itemname {
        result_item = controller.get_item_by_name(&itemname);
    } else {
        if let Some(itemid) = itemx {
            result_item = controller.get_item_by_id(itemid.parse().expect("Error parsing itemid"));
        } else {
            println!("No item name nor item id");
            return;
        }
    }

    if result_item.is_empty() {
        println!("No item in database movie");
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
                if musers_neighbors[i].scores().contains_key(&item.id) {
                    //println!("{} {}", musers_neighbors[i].name(), pearson_results[i]);
                    let score_item = musers_neighbors[i].scores().get(&item.id).unwrap().clone();
                    total_influence += pearson_results[i];
                    prediction += score_item * pearson_results[i];
                }
            }

            prediction /= total_influence;
            println!("Prediction for user ({}) with item {}({}) is: {}", user.id, item.id, item.name, prediction);
        }
    }
}

pub fn knn_recommend(k: i32, xid: Option<String>, userx: Option<String>, distance_type: Distance) {
    let controller = SmallMovielensController::new();
    
    let mut result_user = Vec::new();

    if let Some(username) = userx {
        result_user = controller.get_user_by_name(&username);
    } else {
        if let Some(userid) = xid {
            result_user = controller.get_user_by_id(userid.parse().expect("Error parsing useris"));
        } else {
            println!("No user name nor user id");
            return;
        }
    }
    if result_user.is_empty() {
        println!("No user in database movie");
        return;
    }

    let all_scores = controller.get_all_scores();
    for user in &result_user {
        let mut recommendations = HashMap::new();
        let neighbors = distances::knn(k, user.id, &all_scores, distance_type.clone());
        let mut musers_neighbors= Vec::new();
        for n in &neighbors {
            musers_neighbors.push(controller.get_user_by_id(n.id)[0].clone());
        }
        
        let mut pearson_results = Vec::new();
        let mut total_distance = 0.0;
        for muser in &musers_neighbors {
            let pearson_comp = distances::pearson_correlation_between(&user.scores(), &muser.scores());
            pearson_results.push(pearson_comp);
            total_distance += pearson_comp;
        }

        for i in 0..musers_neighbors.len() {
            let weight = pearson_results[i]/total_distance;
            for (itemid, score) in musers_neighbors[i].scores() {
                if !user.scores().contains_key(&itemid) {
                    if recommendations.contains_key(&itemid) {
                        recommendations.insert(itemid, recommendations.get(&itemid).unwrap() + (weight * score));
                    } else {
                        recommendations.insert(itemid, weight * score);
                    }
                }
            }
        }

        let mut result = Vec::new();
        for (itemid, score) in recommendations {
            result.push(distances::TargetToOther{id: itemid, distance: score});
        }

        result.sort();
        result.reverse();

        for it in result {
            let item = &controller.get_item_by_id(it.id)[0];
            println!("Recommend for user ({}) the item {}({}) with score: {}", user.id, item.id, item.name, it.distance);
        }
    }
}
