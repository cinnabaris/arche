table! {
    ar_internal_metadata (key) {
        key -> Varchar,
        value -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    attachments (id) {
        id -> Bigint,
        user_id -> Bigint,
        name -> Varchar,
        size -> Varchar,
        mime_type -> Varchar,
        url -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    cards (id) {
        id -> Bigint,
        title -> Varchar,
        body -> Varchar,
        media_type -> Varchar,
        action -> Varchar,
        href -> Varchar,
        logo -> Varchar,
        loc -> Varchar,
        lang -> Varchar,
        sort -> Tinyint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    friend_links (id) {
        id -> Bigint,
        title -> Varchar,
        home -> Varchar,
        logo -> Varchar,
        sort -> Tinyint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    leave_words (id) {
        id -> Bigint,
        body -> Text,
        media_type -> Varchar,
        created_at -> Datetime,
    }
}

table! {
    links (id) {
        id -> Bigint,
        href -> Varchar,
        label -> Varchar,
        loc -> Varchar,
        lang -> Varchar,
        x -> Tinyint,
        y -> Tinyint,
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
    logs (id) {
        id -> Bigint,
        user_id -> Bigint,
        ip -> Varchar,
        message -> Varchar,
        created_at -> Datetime,
    }
}

table! {
    notices (id) {
        id -> Bigint,
        user_id -> Bigint,
        title -> Varchar,
        body -> Varchar,
        media_type -> Varchar,
        read -> Bool,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    policies (id) {
        id -> Bigint,
        user_id -> Bigint,
        role_id -> Bigint,
        nbf -> Date,
        exp -> Date,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    roles (id) {
        id -> Bigint,
        name -> Varchar,
        resource_type -> Varchar,
        resource_id -> Bigint,
        created_at -> Datetime,
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

table! {
    survey_fields (id) {
        id -> Bigint,
        form_id -> Bigint,
        name -> Varchar,
        label -> Varchar,
        options -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Varchar,
        required -> Bool,
        sort -> Tinyint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    survey_forms (id) {
        id -> Bigint,
        user_id -> Bigint,
        title -> Varchar,
        body -> Text,
        media_type -> Varchar,
        uid -> Varchar,
        mode -> Varchar,
        nbf -> Date,
        exp -> Date,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    survey_records (id) {
        id -> Bigint,
        field_id -> Bigint,
        order -> Varchar,
        value -> Nullable<Text>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    survey_subscribers (id) {
        id -> Bigint,
        form_id -> Bigint,
        email -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    users (id) {
        id -> Bigint,
        name -> Varchar,
        email -> Varchar,
        uid -> Varchar,
        provider_type -> Varchar,
        provider_id -> Varchar,
        logo -> Varchar,
        sign_in_count -> Bigint,
        current_sign_in_at -> Nullable<Datetime>,
        current_sign_in_ip -> Nullable<Varchar>,
        last_sign_in_at -> Nullable<Datetime>,
        last_sign_in_ip -> Nullable<Varchar>,
        locked_at -> Nullable<Datetime>,
        deleted_at -> Nullable<Datetime>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    votes (id) {
        id -> Bigint,
        point -> Bigint,
        resource_type -> Varchar,
        resource_id -> Bigint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(
    ar_internal_metadata,
    attachments,
    cards,
    friend_links,
    leave_words,
    links,
    locales,
    logs,
    notices,
    policies,
    roles,
    schema_migrations,
    settings,
    survey_fields,
    survey_forms,
    survey_records,
    survey_subscribers,
    users,
    votes,
);
