use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::hash::Hash;
use std::cmp::Reverse;

pub const DISABLE: f64 = f64::INFINITY;
#[derive(Debug, Clone)]
pub enum Distance {
    Manhattan,
    Euclidean,
    Minkowski(i32),
    Pearson,
    Cosine,
    JaccardDist,
    JaccardSim
}

pub struct TargetToOther<U> {
    pub id: U,
    pub distance: f64
}

impl<U> Eq for TargetToOther<U> {}

impl<U> PartialEq for TargetToOther<U> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<U> PartialOrd for TargetToOther<U> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl<U> Ord for TargetToOther<U> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.partial_cmp(&other.distance).unwrap()
    }
}



pub fn minkowski_distance_between<I:Hash+Eq+Clone>(x_scores: &HashMap<I, f64>, y_scores: &HashMap<I, f64>, grade: i32) -> f64 {
    let mut distance = 0.0;
    let mut intersection = false;

    for (item, x) in x_scores {
        if let Some(y) = y_scores.get(item) {
            distance += (x - y).abs().powi(grade);
            intersection = true;
        }
    }

    if !intersection {return DISABLE;}

    distance.powf(1.0/(grade as f64))
}

pub fn euclidean_distance_between<I:Hash+Eq+Clone>(x_scores: &HashMap<I, f64>, y_scores: &HashMap<I, f64>) -> f64 {
    minkowski_distance_between(x_scores, y_scores, 2)
}

pub fn manhattan_distance_between<I:Hash+Eq+Clone>(x_scores: &HashMap<I, f64>, y_scores: &HashMap<I, f64>) -> f64 {
    minkowski_distance_between(x_scores, y_scores, 1)
}

pub fn pearson_correlation_between<I:Hash+Eq+Clone>(x_scores: &HashMap<I, f64>, y_scores: &HashMap<I, f64>) -> f64 {
    let mut dot_product = 0.0;
    let mut sumation_x_score = 0.0;
    let mut sumation_y_score = 0.0;
    let mut dot_square_x = 0.0;
    let mut dot_square_y = 0.0;
    let mut intersection = 0.0;

    for (item, x) in x_scores {
        if let Some(y) = y_scores.get(item) {
            dot_product += x * y;
            sumation_x_score += x;
            sumation_y_score += y;
            dot_square_x += x.powi(2); 
            dot_square_y += y.powi(2);
            intersection += 1.0;
        }
    }

    let first = (sumation_x_score * sumation_y_score) / intersection;
    let secondx = sumation_x_score.powi(2) / intersection;
    let secondy = sumation_y_score.powi(2) /intersection;
    let thirdx = (dot_square_x - secondx).sqrt();
    let thirdy = (dot_square_y - secondy).sqrt();

    if (thirdx * thirdy) == 0.0 || intersection == 0.0{
        return DISABLE;
    }

    (dot_product - first)/(thirdx * thirdy)
}

pub fn cosine_similarity_between<I:Hash+Eq+Clone>(x_scores: &HashMap<I, f64>, y_scores: &HashMap<I, f64>) -> f64 {
    let mut dot_product = 0.0;
    let mut len_x_score = 0.0;
    let mut len_y_score = 0.0;

    for (item, x) in x_scores {
        if let Some(y) = y_scores.get(item) {
            dot_product += x * y;
            len_x_score += x.powi(2);
            len_y_score += y.powi(2);
        }
    }

    if len_x_score.sqrt() == 0.0 || len_y_score.sqrt() == 0.0 {
        return DISABLE;
    }

    dot_product/(len_x_score.sqrt() * len_y_score.sqrt())
}

pub fn jaccard_similarity_between<I:Hash+Eq+Clone>(x_scores: &HashMap<I, f64>, y_scores: &HashMap<I, f64>) -> f64 {
    let mut intersection = 0;

    for item in x_scores.keys() {
        if y_scores.contains_key(item) { intersection += 1 }
    }

    let union = ((x_scores.keys().len() - intersection) + y_scores.keys().len()) as f64;

    if union == 0.0 {return  DISABLE;}

    intersection as f64/union as f64
}

pub fn jaccard_distance_between<I:Hash+Eq+Clone>(x_scores: &HashMap<I, f64>, y_scores: &HashMap<I, f64>) -> f64 {
    let j_sim = jaccard_similarity_between(x_scores, y_scores);
    if j_sim == DISABLE {return  DISABLE;}
    1.0 - j_sim
}

pub fn knn<I:Hash+Eq+Clone>(k: i32, user: i32, scores: &HashMap<i32, HashMap<I, f64>>, distance_type: Distance) -> Vec<TargetToOther<i32>> {
    match distance_type {
        Distance::Manhattan |
        Distance::Euclidean |
        Distance::Minkowski(_) => {
            knn_max_heap(k, user, scores, distance_type)
        }
        Distance::Pearson |
        Distance::Cosine |
        Distance::JaccardDist |
        Distance::JaccardSim => {
            knn_min_heap(k, user, scores, distance_type)
        }
    }
}

fn knn_max_heap<I:Hash+Eq+Clone>(k: i32, user: i32, all_scores: &HashMap<i32, HashMap<I, f64>>, distance_type: Distance) -> Vec<TargetToOther<i32>>{
    let mut max_heap:BinaryHeap<TargetToOther<i32>> = BinaryHeap::new();
    
    let user_scores = all_scores.get(&user).expect("Error get target in maxheap");

    for (other, scores) in all_scores {
        if user == *other {
            continue;
        }

        let dist = match distance_type {
            Distance::Manhattan => {
                manhattan_distance_between(user_scores, scores)
            }
            Distance::Euclidean => {
                euclidean_distance_between(user_scores, scores)
            }
            Distance::Minkowski(grade) => {
                minkowski_distance_between(user_scores, scores, grade)
            }
            Distance::Pearson => {DISABLE}
            Distance::Cosine => {DISABLE}
            Distance::JaccardDist => {
                jaccard_distance_between(user_scores, scores)
            }
            Distance::JaccardSim => {DISABLE}
        };

        if dist == DISABLE {continue;}

        let target_to_other = TargetToOther{id: *other, distance: dist};

        if (max_heap.len() as i32) == k {
            if max_heap.peek().unwrap().distance > dist {
                max_heap.pop();
                max_heap.push(target_to_other);
            }
        } else {
            max_heap.push(target_to_other);
        }
    }
    
    max_heap.into_sorted_vec()
}

fn knn_min_heap<I:Hash+Eq+Clone>(k: i32, user: i32, all_scores: &HashMap<i32, HashMap<I, f64>>, distance_type: Distance) -> Vec<TargetToOther<i32>>{
    let mut min_heap:BinaryHeap<Reverse<TargetToOther<i32>>> = BinaryHeap::new();
    
    let user_scores = all_scores.get(&user).expect("Error get target in maxheap");

    for (other, scores) in all_scores {
        if user == *other {
            continue;
        }

        let dist = match distance_type {
            Distance::Manhattan => {DISABLE}
            Distance::Euclidean => {DISABLE}
            Distance::Minkowski(_) => {DISABLE}
            Distance::Pearson => {
                pearson_correlation_between(user_scores, scores)
            }
            Distance::Cosine => {
                cosine_similarity_between(user_scores, scores)
            }
            Distance::JaccardDist => {DISABLE}
            Distance::JaccardSim => {
                jaccard_similarity_between(user_scores, scores)
            }
        };

        if dist == DISABLE {continue;}

        let target_to_other = TargetToOther{id: *other, distance: dist};

        if (min_heap.len() as i32) == k {
            if min_heap.peek().unwrap().0.distance < dist {
                min_heap.pop();
                min_heap.push(Reverse(target_to_other));
            }
        } else {
            min_heap.push(Reverse(target_to_other));
        }
    }
    
    let temp = min_heap.into_sorted_vec();

    let mut result = Vec::new();
    for it in temp {
        result.push(it.0);
    }

    result
}

