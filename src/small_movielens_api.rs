use std::collections::HashMap;
use std::collections::HashSet;

use crate::distances_user;
use crate::distances_item;
use crate::distances_user::Distance;

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

    let distance = distances_user::match_distances(&userx.scores(), &usery.scores(), distance_type.clone());

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

    let neighbors = distances_user::knn(k, userx.id, &scores, distance_type.clone());

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
        let neighbors = distances_user::knn(k, user.id, &scores, distance_type.clone());
        let mut musers_neighbors= Vec::new();
        for n in &neighbors {
            musers_neighbors.push(controller.get_user_by_id(n.id)[0].clone());
        }
        
        let mut pearson_results = Vec::new();
        
        for muser in &musers_neighbors {
            let pearson_comp = distances_user::pearson_correlation_between(&user.scores(), &muser.scores());
            pearson_results.push(pearson_comp);
        }

        for item in &result_item {
            let mut total_influence = 0.0;
            let mut prediction = 0.0;
            for i in 0..neighbors.len() {
                if pearson_results[i] == distances_user::DISABLE {continue;}
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

pub fn distance_item_by_id(idx: &str, idy: &str) {
    let controller = SmallMovielensController::new();

    let all_scores = controller.get_all_scores();    

    let idx = idx.parse().expect("Error parsing item id");
    let idy = idy.parse().expect("Error parsing item id");

    if controller.get_item_by_id(idx).is_empty() || controller.get_item_by_id(idy).is_empty(){
        println!("One or both items are not found in the database");
        return;
    }

    let distance = distances_item::acosine_similarity_between(idx, idy, &all_scores);
    println!("Movie database, Adjusted Cosine Similarity between:");
    println!("Item id({x}) and item id({y}) is: {distance}\n", x = idx, y = idy, distance = distance);
}

pub fn distance_item_by_name(namex: &str, namey: &str) {
    let controller = SmallMovielensController::new();

    let all_scores = controller.get_all_scores();    

    if controller.get_item_by_name(&namex).is_empty() || controller.get_item_by_name(&namey).is_empty(){
        println!("One or both items are not found in the database");
        return;
    }

    let idx = controller.get_item_by_name(&namex)[0].id;
    let idy = controller.get_item_by_name(&namey)[0].id;

    let distance = distances_item::acosine_similarity_between(idx, idy, &all_scores);
    println!("Movie database, Adjusted Cosine Similarity between:");
    println!("Item name({x}) and item name({y}) is: {distance}\n", x = namex, y = namey, distance = distance);
}


pub fn matrix_adjusted_cosine() -> (HashMap<String, usize>,Vec<Vec<f64>>){
    let controller = SmallMovielensController::new();

    let mut items = Vec::new();
    let mut hash_item = HashMap::new();

    let all_scores = controller.get_all_scores();
    let mut matrix = Vec::new();
    let mut order = HashMap::new();

    let mut remainders = HashMap::new();

    let mut it = 0;
    for (user, scores) in all_scores {
        let sum:f64 = scores.values().sum();
        let mean = sum/scores.len() as f64;
        
        remainders.insert(user, HashMap::new());

        for (item, score) in scores {
            if !hash_item.contains_key(&item) {
                let item_name = controller.get_item_by_id(item);
                items.push(item);
                hash_item.insert(item, HashSet::new());
                order.insert(item_name[0].name.clone(), it);
                it += 1;
            }
            let rem = score - mean;
            hash_item.get_mut(&item).unwrap().insert(user);
            remainders.get_mut(&user).unwrap().insert(item, rem);
        }
    }
    
    for _it in &items {
        matrix.push(vec![f64::INFINITY; items.len()]);
    }

    for it in 0..items.len() {
        for it2 in it+1..items.len() {
            let mut numerator = 0.0;
            let mut denominator1 = 0.0;
            let mut denominator2 = 0.0;

            let idx = items[it];
            let idy = items[it2];
            let users_int = hash_item[&idx].intersection(&hash_item[&idy]);
            for user in users_int {
                let remx = remainders[user][&idx];
                let remy = remainders[user][&idy];
                numerator += remx * remy;
                denominator1 += remx.powi(2);
                denominator2 += remy.powi(2);
            }
            matrix[it][it2] = numerator/(denominator1.sqrt()*denominator2.sqrt()); 
        }
    }

    (order, matrix)
}

pub fn get_from_matrix(namex: &str, namey: &str, order: HashMap<String, usize>, matrix: Vec<Vec<f64>>) -> f64{
    let x = *order.get(&String::from(namex)).unwrap();
    let y = *order.get(&String::from(namey)).unwrap();
    let distance = matrix[x][y].min(matrix[y][x]);
    println!("{} {} {}", namex, namey, distance);
    distance
}