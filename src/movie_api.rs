use std::collections::HashMap;

use crate::distances;
use crate::distances::Distance;

use generic_controller::{GenericController, User};
use movie_controller::m_controller::MovieController;


pub fn distance_by_id(idx:String, idy:String, distance_type: Distance) {
    let controller = MovieController::new();

    let usersx = controller.get_user_by_id(idx.parse().expect("First id user in movie manhattan not parsed"));
    let usersy = controller.get_user_by_id(idy.parse().expect("Second id user in movie manhattan not parsed"));

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
    println!("Movie database, {:?} distance between:", distance_type);
    println!("User id({x}) and user id({y}) is: {distance}\n", x = userx.id, y = usery.id, distance = distance);
}

pub fn distance_by_name(namex:String, namey:String, distance_type: Distance) {
    let controller = MovieController::new();

    let usersx = controller.get_user_by_name(&namex);
    let usersy = controller.get_user_by_name(&namey);

    if usersx.is_empty() {
        println!("No user with name {} found!", namex);
        return;
    }
    if usersy.is_empty() {
        println!("No user with name {} found!", namey);
        return;
    }

    for x in &usersx {
        for y in &usersy {
            let distance = match distance_type {
                Distance::Manhattan => {
                    distances::manhattan_distance_between::<i32>(&x.scores(), &y.scores())
                }
                Distance::Euclidean => {
                    distances::euclidean_distance_between::<i32>(&x.scores(), &y.scores())
                }
                Distance::Minkowski(grade) => {
                    distances::minkowski_distance_between(&x.scores(), &y.scores(), grade)
                }
                Distance::Pearson => {
                    distances::pearson_correlation_between(&x.scores(), &y.scores())
                }
                Distance::Cosine => {
                    distances::cosine_similarity_between(&x.scores(), &y.scores())
                }
                Distance::JaccardDist => {
                    distances::jaccard_distance_between(&x.scores(), &y.scores())
                }
                Distance::JaccardSim => {
                    distances::jaccard_similarity_between(&x.scores(), &y.scores())
                }
            };
            println!("Movie database, {:?} distance between:", distance_type);
            println!("User {namex}({idx}) and user {namey}({idy}) is: {distance}\n", namex = x.name, idx = x.id, namey = y.name, idy = y.id, distance = distance);
        }
    }
}

pub fn knn_by_id(k: i32, idx: String, distance_type: Distance) {
    let controller = MovieController::new();

    let usersx = controller.get_user_by_id(idx.parse().expect("First id user in movie knn not parsed"));

    if usersx.is_empty() {
        println!("No user with id {} found!", idx);
        return;
    }

    let userx = &usersx[0];
    let scores = controller.get_all_scores(); 
    let neighbors = distances::knn(k, userx.id, &scores, distance_type.clone());

    println!("Movie database, K({}) nearest neighbors for {}({}) with {:?}", k, userx.name, userx.id, distance_type.clone());
    for it in neighbors {
        let other = &controller.get_user_by_id(it.id)[0];
        println!("With {}({}) distance is {}", other.name, other.id, it.distance);
    }
    println!();
}

pub fn knn_by_name(k: i32, namex: String, distance_type: Distance) {
    let controller = MovieController::new();

    let usersx = controller.get_user_by_name(&namex);

    if usersx.is_empty() {
        println!("No user with name {} found!", namex);
        return;
    }

    let scores = controller.get_all_scores(); 
    for u in usersx {
        let neighbors = distances::knn(k, u.id, &scores, distance_type.clone());

        println!("Movie database, K({}) nearest neighbors for user {}({}) with {:?}", k, u.name, u.id, distance_type);
        for it in neighbors {
            let other = &controller.get_user_by_id(it.id)[0];
            println!("With {}({}) distance is {}", other.name, other.id, it.distance);
        }
    }
    println!();
}

pub fn knn_prediction(k: i32, idx: Option<String>, idy: Option<String>, userx: Option<String>, itemy: Option<String>, distance_type: Distance) {
    let controller = MovieController::new();
    
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
                    //println!("{} {}", musers_neighbors[i].name(), pearson_results[i]);
                    let score_item = musers_neighbors[i].scores().get(&item.id).unwrap().clone();
                    total_influence += pearson_results[i];
                    prediction += score_item * pearson_results[i];
                }
            }

            prediction /= total_influence;
            println!("Movie database, prediction for user {}({})", user.name, user.id);
            println!("With item {}({}) score is: {}\n", item.name, item.id, prediction);
        }
    }
}


