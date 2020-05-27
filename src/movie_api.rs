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
    println!("{dist_name:?} Distance between id({x}) and id({y}) is: {distance}", x = userx.id, y = usery.id, distance = distance, dist_name = distance_type);
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
            println!("{dist_name:?} Distance between {namex}({idx}) and {namey}({idy}) is: {distance}", namex = x.name, idx = x.id, namey = y.name, idy = y.id, distance = distance, dist_name = distance_type);
        }
    }
}