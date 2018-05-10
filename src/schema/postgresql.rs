table! {
    attachments (id) {
        id -> Int4,
        title -> Varchar,
        url -> Varchar,
        length -> Int4,
        media_type -> Varchar,
        resource_type -> Varchar,
        resource_id -> Int4,
        user_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    cards (id) {
        id -> Int4,
        title -> Varchar,
        summary -> Text,
        #[sql_name = "type"]
        type_ -> Varchar,
        action -> Varchar,
        href -> Varchar,
        logo -> Varchar,
        loc -> Varchar,
        lang -> Varchar,
        sort -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    cbeta_books (id) {
        id -> Int4,
        uid -> Varchar,
        author -> Varchar,
        publisher -> Varchar,
        title -> Varchar,
        mime_type -> Varchar,
        lang -> Varchar,
        subject -> Nullable<Varchar>,
        description -> Nullable<Text>,
        published_at -> Nullable<Date>,
        cover -> Nullable<Varchar>,
        home -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    cbeta_notes (id) {
        id -> Int4,
        user_id -> Int4,
        book_id -> Int4,
        body -> Text,
        media_type -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    cbeta_pages (id) {
        id -> Int4,
        href -> Int4,
        book_id -> Int4,
        body -> Bytea,
        media_type -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    donate_payments (id) {
        id -> Int4,
        title -> Varchar,
        summary -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        profile -> Text,
        project_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    donate_projects (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        #[sql_name = "type"]
        type_ -> Varchar,
        user_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    forum_categories (id) {
        id -> Int4,
        name -> Varchar,
        color -> Varchar,
        icon -> Varchar,
        description -> Nullable<Varchar>,
        parent_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    forum_posts (id) {
        id -> Int4,
        body -> Text,
        user_id -> Int4,
        post_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    forum_tags (id) {
        id -> Int4,
        name -> Varchar,
        background_color -> Varchar,
        text_color -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    forum_topics (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        user_id -> Int4,
        category_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    forum_topics_tags (id) {
        id -> Int4,
        topic_id -> Int4,
        tag_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    friend_links (id) {
        id -> Int4,
        title -> Varchar,
        home -> Varchar,
        logo -> Varchar,
        sort -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    hotel_logs (id) {
        id -> Int4,
        member_id -> Int4,
        room_id -> Int4,
        action -> Varchar,
        days -> Nullable<Int2>,
        description -> Text,
        created_at -> Timestamp,
    }
}

table! {
    hotel_rooms (id) {
        id -> Int4,
        uid -> Varchar,
        loc -> Varchar,
        floor -> Varchar,
        door -> Varchar,
        bed -> Varchar,
        status -> Int2,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    leave_words (id) {
        id -> Int4,
        body -> Text,
        #[sql_name = "type"]
        type_ -> Varchar,
        created_at -> Timestamptz,
    }
}

table! {
    library_books (id) {
        id -> Int4,
        uid -> Varchar,
        title -> Varchar,
        author -> Varchar,
        publisher -> Varchar,
        status -> Int2,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    library_logs (id) {
        id -> Int4,
        member_id -> Int4,
        book_id -> Int4,
        action -> Varchar,
        days -> Nullable<Int2>,
        description -> Text,
        created_at -> Timestamp,
    }
}

table! {
    links (id) {
        id -> Int4,
        href -> Varchar,
        label -> Varchar,
        loc -> Varchar,
        lang -> Varchar,
        x -> Int4,
        y -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    locales (id) {
        id -> Int4,
        lang -> Varchar,
        code -> Varchar,
        message -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    logs (id) {
        id -> Int4,
        user_id -> Int4,
        ip -> Varchar,
        message -> Varchar,
        created_at -> Timestamptz,
    }
}

table! {
    mail_aliases (id) {
        id -> Int4,
        domain_id -> Int4,
        source -> Varchar,
        destination -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mail_domains (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mail_users (id) {
        id -> Int4,
        domain_id -> Int4,
        email -> Varchar,
        full_name -> Varchar,
        password -> Varchar,
        nbf -> Date,
        exp -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_countries (id) {
        id -> Int4,
        name -> Varchar,
        iso_name -> Varchar,
        numcode -> Int4,
        iso -> Varchar,
        iso3 -> Varchar,
        states_required -> Bool,
        zipcode_required -> Bool,
        updated_at -> Timestamp,
    }
}

table! {
    mall_currencies (id) {
        id -> Int4,
        key -> Varchar,
        iso_code -> Varchar,
        name -> Varchar,
        symbol -> Nullable<Varchar>,
        alternate_symbols -> Varchar,
        subunit -> Nullable<Varchar>,
        subunit_to_unit -> Int4,
        symbol_first -> Bool,
        html_entity -> Nullable<Varchar>,
        decimal_mark -> Bpchar,
        thousands_separator -> Bpchar,
        iso_numeric -> Nullable<Int4>,
        smallest_denomination -> Nullable<Int4>,
        updated_at -> Timestamp,
    }
}

table! {
    mall_states (id) {
        id -> Int4,
        name -> Varchar,
        abbr -> Varchar,
        country_id -> Int4,
        updated_at -> Timestamp,
    }
}

table! {
    mall_zone_members (id) {
        id -> Int4,
        zone_id -> Int4,
        zoneable_type -> Varchar,
        zoneable_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_zones (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        kind -> Varchar,
        default_tax -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    members (id) {
        id -> Int4,
        email -> Varchar,
        name -> Varchar,
        address -> Varchar,
        phone -> Varchar,
        summary -> Nullable<Text>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    policies (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        nbf -> Date,
        exp -> Date,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
        resource_id -> Nullable<Int4>,
        resource_type -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    settings (id) {
        id -> Int4,
        key -> Varchar,
        salt -> Nullable<Bytea>,
        value -> Bytea,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    survey_fields (id) {
        id -> Int4,
        label -> Varchar,
        name -> Varchar,
        value -> Varchar,
        body -> Text,
        #[sql_name = "type"]
        type_ -> Varchar,
        required -> Bool,
        form_id -> Int4,
        sort_order -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    survey_forms (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        #[sql_name = "type"]
        type_ -> Varchar,
        uid -> Varchar,
        mode -> Varchar,
        user_id -> Int4,
        nbf -> Date,
        exp -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    survey_records (id) {
        id -> Int4,
        value -> Text,
        form_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    todo_logs (id) {
        id -> Int4,
        description -> Text,
        begin -> Timestamp,
        end -> Timestamp,
        task_id -> Int4,
        member_id -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    todo_projects (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    todo_tasks (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        status -> Int2,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        uid -> Varchar,
        password -> Nullable<Bytea>,
        provider_id -> Varchar,
        provider_type -> Varchar,
        logo -> Nullable<Varchar>,
        sign_in_count -> Int4,
        current_sign_in_at -> Nullable<Timestamptz>,
        current_sign_in_ip -> Nullable<Varchar>,
        last_sign_in_at -> Nullable<Timestamptz>,
        last_sign_in_ip -> Nullable<Varchar>,
        confirmed_at -> Nullable<Timestamptz>,
        locked_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    votes (id) {
        id -> Int4,
        resource_type -> Varchar,
        resource_id -> Int4,
        point -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    vpn_logs (id) {
        id -> Int4,
        user_id -> Int4,
        trusted_ip -> Nullable<Varchar>,
        trusted_port -> Nullable<Int2>,
        remote_ip -> Nullable<Varchar>,
        remote_port -> Nullable<Int2>,
        start_up -> Timestamp,
        shut_down -> Nullable<Timestamp>,
        received -> Float8,
        send -> Float8,
    }
}

table! {
    vpn_users (id) {
        id -> Int4,
        full_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        summary -> Nullable<Text>,
        online -> Bool,
        enable -> Bool,
        nbf -> Date,
        exp -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    attachments,
    cards,
    cbeta_books,
    cbeta_notes,
    cbeta_pages,
    donate_payments,
    donate_projects,
    forum_categories,
    forum_posts,
    forum_tags,
    forum_topics,
    forum_topics_tags,
    friend_links,
    hotel_logs,
    hotel_rooms,
    leave_words,
    library_books,
    library_logs,
    links,
    locales,
    logs,
    mail_aliases,
    mail_domains,
    mail_users,
    mall_countries,
    mall_currencies,
    mall_states,
    mall_zone_members,
    mall_zones,
    members,
    policies,
    roles,
    settings,
    survey_fields,
    survey_forms,
    survey_records,
    todo_logs,
    todo_projects,
    todo_tasks,
    users,
    votes,
    vpn_logs,
    vpn_users,
);
