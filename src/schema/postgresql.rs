table! {
    locales (id) {
        id -> Int8,
        lang -> Varchar,
        code -> Varchar,
        message -> Text,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

table! {
    logs (id) {
        id -> Int8,
        user_id -> Int8,
        ip -> Varchar,
        message -> Varchar,
        created_at -> Timestamptz,
    }
}

table! {
    policies (id) {
        id -> Int8,
        user_id -> Int8,
        role_id -> Int8,
        nbf -> Date,
        exp -> Date,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

table! {
    roles (id) {
        id -> Int8,
        name -> Varchar,
        resource_id -> Nullable<Int8>,
        resource_type -> Nullable<Varchar>,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

table! {
    settings (id) {
        id -> Int8,
        key -> Varchar,
        salt -> Nullable<Bytea>,
        value -> Bytea,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int8,
        name -> Varchar,
        email -> Varchar,
        uid -> Varchar,
        password -> Nullable<Bytea>,
        provider_id -> Varchar,
        provider_type -> Varchar,
        logo -> Nullable<Varchar>,
        sign_in_count -> Int8,
        current_sign_in_at -> Nullable<Timestamptz>,
        current_sign_in_ip -> Nullable<Varchar>,
        last_sign_in_at -> Nullable<Timestamptz>,
        last_sign_in_ip -> Nullable<Varchar>,
        confirmed_at -> Nullable<Timestamptz>,
        locked_at -> Nullable<Timestamptz>,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

joinable!(logs -> users (user_id));
joinable!(policies -> roles (role_id));
joinable!(policies -> users (user_id));

allow_tables_to_appear_in_same_query!(
    locales,
    logs,
    policies,
    roles,
    settings,
    users,
);
