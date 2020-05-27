use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub enum Distance {
    Manhattan,
    Euclidean,
    Minkowski(i32),
    Pearson,
    Cosine,
    JaccardDist,
    JaccardSim
}

pub fn minkowski_distance_between<I:Hash+Eq+Clone>(x_scores: &HashMap<I, f64>, y_scores: &HashMap<I, f64>, grade: i32) -> f64 {
    let mut distance = 0.0;

    for (item, x) in x_scores {
        if let Some(y) = y_scores.get(item) {
            distance += (x - y).abs().powi(grade);
        }
    }
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

    if (thirdx * thirdy) == 0.0 {
        return 0.0;
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
        return 0.0;
    }

    dot_product/(len_x_score.sqrt() * len_y_score.sqrt())
}

pub fn jaccard_similarity_between<I:Hash+Eq+Clone>(x_scores: &HashMap<I, f64>, y_scores: &HashMap<I, f64>) -> f64 {
    let mut intersection = 0;

    for item in x_scores.keys() {
        if y_scores.contains_key(item) { intersection += 1 }
    }

    let union = ((x_scores.keys().len() - intersection) + y_scores.keys().len()) as f64;

    intersection as f64/union as f64
}

pub fn jaccard_distance_between<I:Hash+Eq+Clone>(x_scores: &HashMap<I, f64>, y_scores: &HashMap<I, f64>) -> f64 {
    1.0 - jaccard_similarity_between(x_scores, y_scores)
}

