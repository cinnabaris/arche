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
    settings (id) {
        id -> Int8,
        key -> Varchar,
        salt -> Nullable<Bytea>,
        value -> Bytea,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    locales,
    settings,
);
