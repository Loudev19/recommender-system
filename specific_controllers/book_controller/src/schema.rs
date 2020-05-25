table! {
    books (id) {
        id -> Varchar,
        title -> Varchar,
        author -> Varchar,
        pubyear -> Varchar,
        publisher -> Varchar,
    }
}

table! {
    scores (id) {
        id -> Int4,
        userid -> Int4,
        bookid -> Varchar,
        score -> Float8,
    }
}

table! {
    users (id) {
        id -> Int4,
        city -> Varchar,
        age -> Nullable<Int4>,
    }
}

joinable!(scores -> books (bookid));
joinable!(scores -> users (userid));

allow_tables_to_appear_in_same_query!(
    books,
    scores,
    users,
);
