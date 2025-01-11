// @generated automatically by Diesel CLI.

diesel::table! {
    tips (id) {
        id -> Uuid,
        for_user_id -> Uuid,
        from_user_id -> Nullable<Uuid>,
        from_guest -> Bool,
        #[max_length = 100]
        from_full_name -> Nullable<Varchar>,
        message -> Nullable<Text>,
        transaction_id -> Uuid,
        #[max_length = 50]
        status -> Varchar,
        available_at -> Timestamp,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    transactions (id) {
        id -> Uuid,
        #[max_length = 50]
        payment_method_code -> Varchar,
        #[max_length = 10]
        currency -> Varchar,
        amount -> Float8,
        #[max_length = 50]
        status -> Varchar,
        #[max_length = 100]
        payment_code -> Nullable<Varchar>,
        qr_string -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_balances (user_id) {
        user_id -> Uuid,
        available -> Nullable<Float8>,
        pending -> Nullable<Float8>,
        withdrawn -> Nullable<Float8>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_oauths (user_id) {
        user_id -> Uuid,
        #[max_length = 100]
        google_id -> Nullable<Varchar>,
        #[max_length = 100]
        discord_id -> Nullable<Varchar>,
        #[max_length = 100]
        x_id -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 50]
        phone -> Nullable<Varchar>,
        #[max_length = 100]
        password -> Varchar,
        #[max_length = 100]
        full_name -> Varchar,
        biography -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(tips -> transactions (transaction_id));
diesel::joinable!(user_balances -> users (user_id));
diesel::joinable!(user_oauths -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    tips,
    transactions,
    user_balances,
    user_oauths,
    users,
);
