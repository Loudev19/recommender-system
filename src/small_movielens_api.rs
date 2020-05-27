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

    println!("Small Movielens database, {:?} distance between:", distance_type);
    println!("User id({x}) and user id({y}) is: {distance}\n", x = userx.id, y = usery.id, distance = distance);
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

    println!("Small movielens database, K({}) nearest neighbors for ({}) with {:?}", k, userx.id, distance_type.clone());
    for it in neighbors {
        let other = &controller.get_user_by_id(it.id)[0];
        println!("With ({}) distance is {}", other.id, it.distance);
    }
    println!();
}

pub fn knn_prediction(k: i32, idx: String, idy: Option<String>, itemy: Option<String>, distance_type: Distance) {
    let controller = SmallMovielensController::new();
    
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
                if musers_neighbors[i].scores().contains_key(&item.id) {
                    //println!("{} {}", musers_neighbors[i].name(), pearson_results[i]);
                    let score_item = musers_neighbors[i].scores().get(&item.id).unwrap().clone();
                    total_influence += pearson_results[i];
                    prediction += score_item * pearson_results[i];
                }
            }

            prediction /= total_influence;
            println!("Small Movielens database, prediction for user ({})", user.id);
            println!("With item {}({}) score is: {}\n", item.name, item.id, prediction);
        }
    }
}

pub fn knn_recommend(k: i32, idx: Option<String>, userx: Option<String>, distance_type: Distance) {
    let controller = SmallMovielensController::new();
    
    let mut result_user = Vec::new();

    if let Some(username) = userx {
        result_user = controller.get_user_by_name(&username);
    } else {
        if let Some(userid) = idx {
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

        println!("Small Movielens database, recommendations for user ({})", user.id);
        let mut i = 0;
        for it in result {
            if i < 5 {
                let item = &controller.get_item_by_id(it.id)[0];
                println!("{} Item {}({}) with score: {}", i+1, item.name, item.id, it.distance);
                i += 1;
            } else {
                println!();
                return;
            }
        }
    }
}
