table! {
    ar_internal_metadata (key) {
        key -> Varchar,
        value -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    attachments (id) {
        id -> Int8,
        user_id -> Int8,
        name -> Varchar,
        size -> Varchar,
        mime_type -> Varchar,
        url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    cards (id) {
        id -> Int8,
        title -> Varchar,
        body -> Text,
        media_type -> Varchar,
        action -> Varchar,
        href -> Varchar,
        logo -> Varchar,
        loc -> Varchar,
        lang -> Varchar,
        position -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    caring_managers (id) {
        id -> Int8,
        topic_id -> Int8,
        user_id -> Int8,
        created_at -> Timestamp,
    }
}

table! {
    caring_posts (id) {
        id -> Int8,
        topic_id -> Int8,
        member_id -> Int8,
        who -> Varchar,
        method -> Varchar,
        address -> Varchar,
        progress -> Int2,
        timezone -> Varchar,
        remind -> Nullable<Varchar>,
        date -> Date,
        begin -> Time,
        end -> Time,
        body -> Text,
        media_type -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    caring_posts_members (id) {
        id -> Int8,
        post_id -> Int8,
        member_id -> Int8,
        created_at -> Timestamp,
    }
}

table! {
    caring_tags (id) {
        id -> Int8,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    caring_topics (id) {
        id -> Int8,
        member_id -> Int8,
        title -> Varchar,
        body -> Text,
        media_type -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    caring_topics_tags (id) {
        id -> Int8,
        topic_id -> Int8,
        tag_id -> Int8,
        created_at -> Timestamp,
    }
}

table! {
    forum_posts (id) {
        id -> Int8,
        user_id -> Int8,
        topic_id -> Int8,
        post_id -> Nullable<Int8>,
        body -> Text,
        media_type -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    forum_tags (id) {
        id -> Int8,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    forum_topics (id) {
        id -> Int8,
        user_id -> Int8,
        lang -> Varchar,
        title -> Varchar,
        body -> Text,
        media_type -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    forum_topics_tags (id) {
        id -> Int8,
        topic_id -> Int8,
        tag_id -> Int8,
    }
}

table! {
    friend_links (id) {
        id -> Int8,
        title -> Varchar,
        home -> Varchar,
        logo -> Varchar,
        position -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    leave_words (id) {
        id -> Int8,
        body -> Text,
        media_type -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    links (id) {
        id -> Int8,
        href -> Varchar,
        label -> Varchar,
        loc -> Varchar,
        lang -> Varchar,
        x -> Int2,
        y -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    locales (id) {
        id -> Int8,
        lang -> Varchar,
        code -> Varchar,
        message -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    logs (id) {
        id -> Int8,
        user_id -> Int8,
        ip -> Varchar,
        message -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    members (id) {
        id -> Int8,
        nick_name -> Varchar,
        real_name -> Varchar,
        phone -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        address -> Nullable<Varchar>,
        line -> Nullable<Varchar>,
        wechat -> Nullable<Varchar>,
        weibo -> Nullable<Varchar>,
        facebook -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    notifications (id) {
        id -> Int8,
        user_id -> Int8,
        url -> Varchar,
        body -> Varchar,
        media_type -> Varchar,
        level -> Varchar,
        read -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    policies (id) {
        id -> Int8,
        user_id -> Int8,
        role_id -> Int8,
        nbf -> Date,
        exp -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    roles (id) {
        id -> Int8,
        name -> Varchar,
        resource_type -> Nullable<Varchar>,
        resource_id -> Nullable<Int8>,
        created_at -> Timestamp,
    }
}

table! {
    schema_migrations (version) {
        version -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    settings (id) {
        id -> Int8,
        key -> Varchar,
        value -> Bytea,
        salt -> Nullable<Bytea>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    survey_fields (id) {
        id -> Int8,
        form_id -> Int8,
        name -> Varchar,
        label -> Varchar,
        options -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Varchar,
        required -> Bool,
        sort -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    survey_forms (id) {
        id -> Int8,
        user_id -> Int8,
        title -> Varchar,
        body -> Text,
        media_type -> Varchar,
        uid -> Varchar,
        mode -> Varchar,
        nbf -> Date,
        exp -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    survey_records (id) {
        id -> Int8,
        field_id -> Int8,
        order -> Varchar,
        value -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    survey_subscribers (id) {
        id -> Int8,
        form_id -> Int8,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int8,
        name -> Varchar,
        email -> Varchar,
        password -> Nullable<Bytea>,
        uid -> Varchar,
        provider_type -> Varchar,
        provider_id -> Varchar,
        logo -> Varchar,
        sign_in_count -> Int8,
        current_sign_in_at -> Nullable<Timestamp>,
        current_sign_in_ip -> Nullable<Varchar>,
        last_sign_in_at -> Nullable<Timestamp>,
        last_sign_in_ip -> Nullable<Varchar>,
        confirmed_at -> Nullable<Timestamp>,
        locked_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    votes (id) {
        id -> Int8,
        point -> Int8,
        resource_type -> Varchar,
        resource_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    ar_internal_metadata,
    attachments,
    cards,
    caring_managers,
    caring_posts,
    caring_posts_members,
    caring_tags,
    caring_topics,
    caring_topics_tags,
    forum_posts,
    forum_tags,
    forum_topics,
    forum_topics_tags,
    friend_links,
    leave_words,
    links,
    locales,
    logs,
    members,
    notifications,
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
