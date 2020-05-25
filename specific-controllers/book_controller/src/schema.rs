table! {
    movies (id) {
        id -> Int4,
        title -> Varchar,
    }
}

table! {
    scores (id) {
        id -> Int4,
        user_id -> Int4,
        movie_id -> Int4,
        score -> Float8,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
    }
}

joinable!(scores -> movies (movie_id));
joinable!(scores -> users (user_id));

allow_tables_to_appear_in_same_query!(
    movies,
    scores,
    users,
);
