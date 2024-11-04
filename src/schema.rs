// @generated automatically by Diesel CLI.

diesel::table! {
    attempted_problems (id) {
        id -> Int4,
        #[max_length = 255]
        user_email -> Varchar,
        problem_id -> Int4,
        is_solved -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    problems (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        problem_statement -> Text,
        tags -> Text,
        difficulty -> Int4,
        hint -> Nullable<Text>,
        #[max_length = 255]
        answer -> Varchar,
        solved_count -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        solved -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    attempted_problems,
    problems,
    users,
);
