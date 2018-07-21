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
        body -> Text,
        media_type -> Varchar,
        action -> Varchar,
        href -> Varchar,
        logo -> Varchar,
        loc -> Varchar,
        lang -> Varchar,
        position -> Tinyint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    caring_managers (id) {
        id -> Bigint,
        topic_id -> Bigint,
        user_id -> Bigint,
        created_at -> Datetime,
    }
}

table! {
    caring_posts (id) {
        id -> Bigint,
        topic_id -> Bigint,
        member_id -> Bigint,
        who -> Varchar,
        method -> Varchar,
        address -> Varchar,
        progress -> Tinyint,
        timezone -> Varchar,
        remind -> Nullable<Varchar>,
        date -> Date,
        begin -> Time,
        end -> Time,
        body -> Text,
        media_type -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    caring_posts_members (id) {
        id -> Bigint,
        post_id -> Bigint,
        member_id -> Bigint,
        created_at -> Datetime,
    }
}

table! {
    caring_tags (id) {
        id -> Bigint,
        name -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    caring_topics (id) {
        id -> Bigint,
        member_id -> Bigint,
        title -> Varchar,
        body -> Text,
        media_type -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    caring_topics_tags (id) {
        id -> Bigint,
        topic_id -> Bigint,
        tag_id -> Bigint,
        created_at -> Datetime,
    }
}

table! {
    forum_posts (id) {
        id -> Bigint,
        user_id -> Bigint,
        topic_id -> Bigint,
        post_id -> Nullable<Bigint>,
        body -> Text,
        media_type -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    forum_tags (id) {
        id -> Bigint,
        name -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    forum_topics (id) {
        id -> Bigint,
        user_id -> Bigint,
        lang -> Varchar,
        title -> Varchar,
        body -> Text,
        media_type -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    forum_topics_tags (id) {
        id -> Bigint,
        topic_id -> Bigint,
        tag_id -> Bigint,
    }
}

table! {
    friend_links (id) {
        id -> Bigint,
        title -> Varchar,
        home -> Varchar,
        logo -> Varchar,
        position -> Tinyint,
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
    members (id) {
        id -> Bigint,
        nick_name -> Varchar,
        real_name -> Varchar,
        phone -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        address -> Nullable<Varchar>,
        line -> Nullable<Varchar>,
        wechat -> Nullable<Varchar>,
        skype -> Nullable<Varchar>,
        weibo -> Nullable<Varchar>,
        facebook -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    notifications (id) {
        id -> Bigint,
        user_id -> Bigint,
        url -> Varchar,
        body -> Varchar,
        media_type -> Varchar,
        level -> Varchar,
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
        resource_type -> Nullable<Varchar>,
        resource_id -> Nullable<Bigint>,
        created_at -> Datetime,
    }
}

table! {
    schema_migrations (version) {
        version -> Varchar,
        created_at -> Datetime,
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
        password -> Nullable<Blob>,
        uid -> Varchar,
        provider_type -> Varchar,
        provider_id -> Varchar,
        logo -> Varchar,
        sign_in_count -> Bigint,
        current_sign_in_at -> Nullable<Datetime>,
        current_sign_in_ip -> Nullable<Varchar>,
        last_sign_in_at -> Nullable<Datetime>,
        last_sign_in_ip -> Nullable<Varchar>,
        confirmed_at -> Nullable<Datetime>,
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
