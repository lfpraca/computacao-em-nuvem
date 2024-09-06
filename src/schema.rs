// @generated automatically by Diesel CLI.

diesel::table! {
    order (id) {
        id -> Uuid,
        date -> Timestamptz,
        amount -> Int2,
        state -> Int2,
        user_id -> Uuid,
        attachment_extension -> Varchar,
    }
}

diesel::table! {
    user (id) {
        id -> Uuid,
        phone_number -> Varchar,
        name -> Varchar,
        pass_hash -> Varchar,
        address -> Varchar,
        role -> Int2,
    }
}

diesel::table! {
    user_token (id) {
        #[max_length = 32]
        id -> Bpchar,
        user_id -> Uuid,
    }
}

diesel::joinable!(order -> user (user_id));
diesel::joinable!(user_token -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    order,
    user,
    user_token,
);
