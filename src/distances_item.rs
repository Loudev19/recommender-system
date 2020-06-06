use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum DistanceItem {
    AdjustedCosine
}

pub fn acosine_similarity_between(idx: i32, idy: i32, all_scores: &HashMap<i32, HashMap<i32, f64>>) -> f64 {
    let mut numerator = 0.0;
    let mut denominator1 = 0.0;
    let mut denominator2 = 0.0;
    let mut means = HashMap::new();
    for (user, scores) in all_scores {
        let mut sum = 0.0;
        for (item, score) in scores {
            sum += score;
        }
        means.insert(user, sum/scores.len() as f64);

        if scores.contains_key(&idx) && scores.contains_key(&idy){
            let mean = means.get(user).expect("Error get user");
            let remx = scores.get(&idx).expect("Error getting idx") - mean;
            let remy = scores.get(&idy).expect("Error getting idx") - mean;
            numerator += remx * remy;
            denominator1 += remx.powi(2);
            denominator2 += remy.powi(2);
        }
    }
    numerator/(denominator1.sqrt()*denominator2.sqrt())
}
