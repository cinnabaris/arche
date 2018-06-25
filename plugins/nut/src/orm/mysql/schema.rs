table! {
    ar_internal_metadata (key) {
        key -> Varchar,
        value -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    locales (id) {
        id -> Bigint,
        lang -> Varchar,
        code -> Varchar,
        message -> Text,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    schema_migrations (version) {
        version -> Varchar,
    }
}

table! {
    settings (id) {
        id -> Bigint,
        key -> Varchar,
        value -> Blob,
        salt -> Nullable<Blob>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(
    ar_internal_metadata,
    locales,
    schema_migrations,
    settings,
);
