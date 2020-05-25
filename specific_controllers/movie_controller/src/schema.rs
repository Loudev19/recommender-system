table! {
    movies (id) {
        id -> Int4,
        title -> Varchar,
    }
}

table! {
    scores (id) {
        id -> Int4,
        userid -> Int4,
        movieid -> Int4,
        score -> Float8,
    }
}

table! {
    users (id) {
        id -> Int4,
        uname -> Varchar,
    }
}

joinable!(scores -> movies (movieid));
joinable!(scores -> users (userid));

allow_tables_to_appear_in_same_query!(
    movies,
    scores,
    users,
);
