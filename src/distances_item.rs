use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum DistanceItem {
    AdjustedCosine
}

pub fn acosine_similarity_between(idx: i32, idy: i32, all_scores: &HashMap<i32, HashMap<i32, f64>>) -> f64 {
    let mut denominator_remainders = HashMap::new();
    let mut numerator = 0.0;
    for (user, scores) in all_scores {
        if scores.contains_key(&idx) && scores.contains_key(&idy){
            let mut sum = 0.0;
            let mut intersection = 0.0;
            for (item, score) in scores {
                sum += score;
                intersection += 1.0;
            }
            let mean = sum/intersection;
            let mut by_user = 1.0;

            let rem = scores.get(&idx).expect("Error parsing idx") - mean;
            by_user *= rem;
            if !denominator_remainders.contains_key(&idx) {
                denominator_remainders.insert(idx, vec![rem]);
            } else {
                denominator_remainders.entry(idx).or_insert(Vec::new()).push(rem);
            }

            let rem = scores.get(&idy).expect("") - mean;
            by_user *= rem;  
            if !denominator_remainders.contains_key(&idy) {
                denominator_remainders.insert(idy, vec![rem]);
            } else {
                denominator_remainders.entry(idy).or_insert(Vec::new()).push(rem);
            }

            numerator += by_user;
        }
    }

    let mut denominator = 1.0;
    for (item, rems) in denominator_remainders {
        let mut by_item = 0.0;
        for rem in rems {
            by_item += rem.powi(2);
        }
        denominator *= by_item.sqrt();
    }

    numerator/denominator
}

pub fn matrix_adjusted_cosine(all_scores: &HashMap<i32, HashMap<i32, f64>>){
    
}