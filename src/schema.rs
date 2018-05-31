table! {
    active_storage_attachments (id) {
        id -> Int8,
        name -> Varchar,
        record_type -> Varchar,
        record_id -> Int8,
        blob_id -> Int8,
        created_at -> Timestamp,
    }
}

table! {
    active_storage_blobs (id) {
        id -> Int8,
        key -> Varchar,
        filename -> Varchar,
        content_type -> Nullable<Varchar>,
        metadata -> Nullable<Text>,
        byte_size -> Int8,
        checksum -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    api_keys (id) {
        id -> Int4,
        key -> Varchar,
        user_id -> Nullable<Int4>,
        created_by_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        allowed_ips -> Nullable<Array<Inet>>,
        hidden -> Bool,
    }
}

table! {
    application_requests (id) {
        id -> Int4,
        date -> Date,
        req_type -> Int4,
        count -> Int4,
    }
}

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
    badge_groupings (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        position -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    badges (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        badge_type_id -> Int4,
        grant_count -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        allow_title -> Bool,
        multiple_grant -> Bool,
        icon -> Nullable<Varchar>,
        listable -> Nullable<Bool>,
        target_posts -> Nullable<Bool>,
        query -> Nullable<Text>,
        enabled -> Bool,
        auto_revoke -> Bool,
        badge_grouping_id -> Int4,
        trigger -> Nullable<Int4>,
        show_posts -> Bool,
        system -> Bool,
        image -> Nullable<Varchar>,
        long_description -> Nullable<Text>,
    }
}

table! {
    badge_types (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
    categories (id) {
        id -> Int4,
        name -> Varchar,
        color -> Varchar,
        topic_id -> Nullable<Int4>,
        topic_count -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Int4,
        topics_year -> Nullable<Int4>,
        topics_month -> Nullable<Int4>,
        topics_week -> Nullable<Int4>,
        slug -> Varchar,
        description -> Nullable<Text>,
        text_color -> Varchar,
        read_restricted -> Bool,
        auto_close_hours -> Nullable<Float8>,
        post_count -> Int4,
        latest_post_id -> Nullable<Int4>,
        latest_topic_id -> Nullable<Int4>,
        position -> Nullable<Int4>,
        parent_category_id -> Nullable<Int4>,
        posts_year -> Nullable<Int4>,
        posts_month -> Nullable<Int4>,
        posts_week -> Nullable<Int4>,
        email_in -> Nullable<Varchar>,
        email_in_allow_strangers -> Nullable<Bool>,
        topics_day -> Nullable<Int4>,
        posts_day -> Nullable<Int4>,
        allow_badges -> Bool,
        name_lower -> Varchar,
        auto_close_based_on_last_post -> Nullable<Bool>,
        topic_template -> Nullable<Text>,
        contains_messages -> Nullable<Bool>,
        sort_order -> Nullable<Varchar>,
        sort_ascending -> Nullable<Bool>,
        uploaded_logo_id -> Nullable<Int4>,
        uploaded_background_id -> Nullable<Int4>,
        topic_featured_link_allowed -> Nullable<Bool>,
        all_topics_wiki -> Bool,
        show_subcategory_list -> Nullable<Bool>,
        num_featured_topics -> Nullable<Int4>,
        default_view -> Nullable<Varchar>,
        subcategory_list_style -> Nullable<Varchar>,
        default_top_period -> Nullable<Varchar>,
        mailinglist_mirror -> Bool,
        suppress_from_latest -> Nullable<Bool>,
        minimum_required_tags -> Nullable<Int4>,
    }
}

table! {
    categories_web_hooks (id) {
        id -> Int4,
        web_hook_id -> Int8,
        category_id -> Int8,
    }
}

table! {
    category_custom_fields (id) {
        id -> Int4,
        category_id -> Int4,
        name -> Varchar,
        value -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    category_featured_topics (id) {
        category_id -> Int4,
        topic_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        rank -> Int4,
        id -> Int8,
    }
}

table! {
    category_groups (id) {
        id -> Int4,
        category_id -> Int4,
        group_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        permission_type -> Nullable<Int4>,
    }
}

table! {
    category_search_data (category_id) {
        category_id -> Int4,
        search_data -> Nullable<Tsvector>,
        raw_data -> Nullable<Text>,
        locale -> Nullable<Text>,
        version -> Nullable<Int4>,
    }
}

table! {
    category_tag_groups (id) {
        id -> Int4,
        category_id -> Int4,
        tag_group_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    category_tags (id) {
        id -> Int4,
        category_id -> Int4,
        tag_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    category_tag_stats (id) {
        id -> Int8,
        category_id -> Int8,
        tag_id -> Int8,
        topic_count -> Int4,
    }
}

table! {
    category_users (id) {
        id -> Int4,
        category_id -> Int4,
        user_id -> Int4,
        notification_level -> Int4,
    }
}

table! {
    cbeta_books (id) {
        id -> Int4,
        title -> Varchar,
        identifier -> Varchar,
        language -> Varchar,
        creator -> Varchar,
        publisher -> Nullable<Varchar>,
        subject -> Nullable<Varchar>,
        description -> Nullable<Text>,
        published_at -> Nullable<Timestamp>,
        version -> Varchar,
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
        book_id -> Int4,
        name -> Varchar,
        href -> Varchar,
        body -> Nullable<Bytea>,
        media_type -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    child_themes (id) {
        id -> Int4,
        parent_theme_id -> Nullable<Int4>,
        child_theme_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    color_scheme_colors (id) {
        id -> Int4,
        name -> Varchar,
        hex -> Varchar,
        color_scheme_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    color_schemes (id) {
        id -> Int4,
        name -> Varchar,
        version -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        via_wizard -> Bool,
        base_scheme_id -> Nullable<Varchar>,
        theme_id -> Nullable<Int4>,
    }
}

table! {
    custom_emojis (id) {
        id -> Int4,
        name -> Varchar,
        upload_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    developers (id) {
        id -> Int4,
        user_id -> Int4,
    }
}

table! {
    directory_items (id) {
        id -> Int4,
        period_type -> Int4,
        user_id -> Int4,
        likes_received -> Int4,
        likes_given -> Int4,
        topics_entered -> Int4,
        topic_count -> Int4,
        post_count -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        days_visited -> Int4,
        posts_read -> Int4,
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
    drafts (id) {
        id -> Int4,
        user_id -> Int4,
        draft_key -> Varchar,
        data -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        sequence -> Int4,
        revisions -> Int4,
    }
}

table! {
    draft_sequences (id) {
        id -> Int4,
        user_id -> Int4,
        draft_key -> Varchar,
        sequence -> Int4,
    }
}

table! {
    email_change_requests (id) {
        id -> Int4,
        user_id -> Int4,
        old_email -> Varchar,
        new_email -> Varchar,
        old_email_token_id -> Nullable<Int4>,
        new_email_token_id -> Nullable<Int4>,
        change_state -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    email_logs (id) {
        id -> Int4,
        to_address -> Varchar,
        email_type -> Varchar,
        user_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        reply_key -> Nullable<Varchar>,
        post_id -> Nullable<Int4>,
        topic_id -> Nullable<Int4>,
        skipped -> Nullable<Bool>,
        skipped_reason -> Nullable<Varchar>,
        bounce_key -> Nullable<Varchar>,
        bounced -> Bool,
        message_id -> Nullable<Varchar>,
    }
}

table! {
    email_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        email -> Varchar,
        token -> Varchar,
        confirmed -> Bool,
        expired -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    embeddable_hosts (id) {
        id -> Int4,
        host -> Varchar,
        category_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        path_whitelist -> Nullable<Varchar>,
        class_name -> Nullable<Varchar>,
    }
}

table! {
    facebook_user_infos (id) {
        id -> Int4,
        user_id -> Int4,
        facebook_user_id -> Int8,
        username -> Nullable<Varchar>,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        gender -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        link -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        avatar_url -> Nullable<Varchar>,
        about_me -> Nullable<Text>,
        location -> Nullable<Varchar>,
        website -> Nullable<Text>,
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
    friendly_id_slugs (id) {
        id -> Int4,
        slug -> Varchar,
        sluggable_id -> Int4,
        sluggable_type -> Nullable<Varchar>,
        scope -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    github_user_infos (id) {
        id -> Int4,
        user_id -> Int4,
        screen_name -> Varchar,
        github_user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    given_daily_likes (id) {
        id -> Int4,
        user_id -> Int4,
        likes_given -> Int4,
        given_date -> Date,
        limit_reached -> Bool,
    }
}

table! {
    google_user_infos (id) {
        id -> Int4,
        user_id -> Int4,
        google_user_id -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        gender -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        link -> Nullable<Varchar>,
        profile_link -> Nullable<Varchar>,
        picture -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    group_archived_messages (id) {
        id -> Int4,
        group_id -> Int4,
        topic_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    group_custom_fields (id) {
        id -> Int4,
        group_id -> Int4,
        name -> Varchar,
        value -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    group_histories (id) {
        id -> Int4,
        group_id -> Int4,
        acting_user_id -> Int4,
        target_user_id -> Nullable<Int4>,
        action -> Int4,
        subject -> Nullable<Varchar>,
        prev_value -> Nullable<Text>,
        new_value -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    group_mentions (id) {
        id -> Int4,
        post_id -> Nullable<Int4>,
        group_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    groups (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        automatic -> Bool,
        user_count -> Int4,
        automatic_membership_email_domains -> Nullable<Text>,
        automatic_membership_retroactive -> Nullable<Bool>,
        primary_group -> Bool,
        title -> Nullable<Varchar>,
        grant_trust_level -> Nullable<Int4>,
        incoming_email -> Nullable<Varchar>,
        has_messages -> Bool,
        flair_url -> Nullable<Varchar>,
        flair_bg_color -> Nullable<Varchar>,
        flair_color -> Nullable<Varchar>,
        bio_raw -> Nullable<Text>,
        bio_cooked -> Nullable<Text>,
        allow_membership_requests -> Bool,
        full_name -> Nullable<Varchar>,
        default_notification_level -> Int4,
        visibility_level -> Int4,
        public_exit -> Bool,
        public_admission -> Bool,
        membership_request_template -> Nullable<Text>,
        messageable_level -> Nullable<Int4>,
        mentionable_level -> Nullable<Int4>,
    }
}

table! {
    groups_web_hooks (id) {
        id -> Int4,
        web_hook_id -> Int8,
        group_id -> Int8,
    }
}

table! {
    group_users (id) {
        id -> Int4,
        group_id -> Int4,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        owner -> Bool,
        notification_level -> Int4,
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
    incoming_domains (id) {
        id -> Int4,
        name -> Varchar,
        https -> Bool,
        port -> Int4,
    }
}

table! {
    incoming_emails (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        topic_id -> Nullable<Int4>,
        post_id -> Nullable<Int4>,
        raw -> Nullable<Text>,
        error -> Nullable<Text>,
        message_id -> Nullable<Text>,
        from_address -> Nullable<Text>,
        to_addresses -> Nullable<Text>,
        cc_addresses -> Nullable<Text>,
        subject -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        rejection_message -> Nullable<Text>,
        is_auto_generated -> Nullable<Bool>,
        is_bounce -> Bool,
    }
}

table! {
    incoming_links (id) {
        id -> Int4,
        created_at -> Timestamp,
        user_id -> Nullable<Int4>,
        ip_address -> Nullable<Inet>,
        current_user_id -> Nullable<Int4>,
        post_id -> Int4,
        incoming_referer_id -> Nullable<Int4>,
    }
}

table! {
    incoming_referers (id) {
        id -> Int4,
        path -> Varchar,
        incoming_domain_id -> Int4,
    }
}

table! {
    instagram_user_infos (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        screen_name -> Nullable<Varchar>,
        instagram_user_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    invited_groups (id) {
        id -> Int4,
        group_id -> Nullable<Int4>,
        invite_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    invites (id) {
        id -> Int4,
        invite_key -> Varchar,
        email -> Nullable<Varchar>,
        invited_by_id -> Int4,
        user_id -> Nullable<Int4>,
        redeemed_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        deleted_by_id -> Nullable<Int4>,
        invalidated_at -> Nullable<Timestamp>,
        moderator -> Bool,
        custom_message -> Nullable<Text>,
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
    message_bus (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        context -> Nullable<Varchar>,
        data -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

table! {
    muted_users (id) {
        id -> Int4,
        user_id -> Int4,
        muted_user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    notifications (id) {
        id -> Int4,
        notification_type -> Int4,
        user_id -> Int4,
        data -> Varchar,
        read -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        topic_id -> Nullable<Int4>,
        post_number -> Nullable<Int4>,
        post_action_id -> Nullable<Int4>,
    }
}

table! {
    oauth2_user_infos (id) {
        id -> Int4,
        user_id -> Int4,
        uid -> Varchar,
        provider -> Varchar,
        email -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    onceoff_logs (id) {
        id -> Int4,
        job_name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    optimized_images (id) {
        id -> Int4,
        sha1 -> Varchar,
        extension -> Varchar,
        width -> Int4,
        height -> Int4,
        upload_id -> Int4,
        url -> Varchar,
    }
}

table! {
    permalinks (id) {
        id -> Int4,
        url -> Varchar,
        topic_id -> Nullable<Int4>,
        post_id -> Nullable<Int4>,
        category_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        external_url -> Nullable<Varchar>,
    }
}

table! {
    plugin_store_rows (id) {
        id -> Int4,
        plugin_name -> Varchar,
        key -> Varchar,
        type_name -> Varchar,
        value -> Nullable<Text>,
    }
}

table! {
    post_actions (id) {
        id -> Int4,
        post_id -> Int4,
        user_id -> Int4,
        post_action_type_id -> Int4,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_by_id -> Nullable<Int4>,
        related_post_id -> Nullable<Int4>,
        staff_took_action -> Bool,
        deferred_by_id -> Nullable<Int4>,
        targets_topic -> Bool,
        agreed_at -> Nullable<Timestamp>,
        agreed_by_id -> Nullable<Int4>,
        deferred_at -> Nullable<Timestamp>,
        disagreed_at -> Nullable<Timestamp>,
        disagreed_by_id -> Nullable<Int4>,
    }
}

table! {
    post_action_types (id) {
        name_key -> Varchar,
        is_flag -> Bool,
        icon -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        id -> Int8,
        position -> Int4,
    }
}

table! {
    post_custom_fields (id) {
        id -> Int4,
        post_id -> Int4,
        name -> Varchar,
        value -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    post_details (id) {
        id -> Int4,
        post_id -> Nullable<Int4>,
        key -> Nullable<Varchar>,
        value -> Nullable<Varchar>,
        extra -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    post_replies (id) {
        id -> Int4,
        post_id -> Nullable<Int4>,
        reply_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    post_revisions (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        post_id -> Nullable<Int4>,
        modifications -> Nullable<Text>,
        number -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        hidden -> Bool,
    }
}

table! {
    posts (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        topic_id -> Int4,
        post_number -> Int4,
        raw -> Text,
        cooked -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        reply_to_post_number -> Nullable<Int4>,
        reply_count -> Int4,
        quote_count -> Int4,
        deleted_at -> Nullable<Timestamp>,
        off_topic_count -> Int4,
        like_count -> Int4,
        incoming_link_count -> Int4,
        bookmark_count -> Int4,
        avg_time -> Nullable<Int4>,
        score -> Nullable<Float8>,
        reads -> Int4,
        post_type -> Int4,
        vote_count -> Int4,
        sort_order -> Nullable<Int4>,
        last_editor_id -> Nullable<Int4>,
        hidden -> Bool,
        hidden_reason_id -> Nullable<Int4>,
        notify_moderators_count -> Int4,
        spam_count -> Int4,
        illegal_count -> Int4,
        inappropriate_count -> Int4,
        last_version_at -> Timestamp,
        user_deleted -> Bool,
        reply_to_user_id -> Nullable<Int4>,
        percent_rank -> Nullable<Float8>,
        notify_user_count -> Int4,
        like_score -> Int4,
        deleted_by_id -> Nullable<Int4>,
        edit_reason -> Nullable<Varchar>,
        word_count -> Nullable<Int4>,
        version -> Int4,
        cook_method -> Int4,
        wiki -> Bool,
        baked_at -> Nullable<Timestamp>,
        baked_version -> Nullable<Int4>,
        hidden_at -> Nullable<Timestamp>,
        self_edits -> Int4,
        reply_quoted -> Bool,
        via_email -> Bool,
        raw_email -> Nullable<Text>,
        public_version -> Int4,
        action_code -> Nullable<Varchar>,
        image_url -> Nullable<Varchar>,
        locked_by_id -> Nullable<Int4>,
    }
}

table! {
    post_search_data (post_id) {
        post_id -> Int4,
        search_data -> Nullable<Tsvector>,
        raw_data -> Nullable<Text>,
        locale -> Nullable<Varchar>,
        version -> Nullable<Int4>,
    }
}

table! {
    post_stats (id) {
        id -> Int4,
        post_id -> Nullable<Int4>,
        drafts_saved -> Nullable<Int4>,
        typing_duration_msecs -> Nullable<Int4>,
        composer_open_duration_msecs -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    post_timings (id) {
        id -> Int4,
        topic_id -> Int4,
        post_number -> Int4,
        user_id -> Int4,
        msecs -> Int4,
    }
}

table! {
    post_uploads (id) {
        id -> Int4,
        post_id -> Int4,
        upload_id -> Int4,
    }
}

table! {
    push_subscriptions (id) {
        id -> Int8,
        user_id -> Int4,
        data -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    queued_posts (id) {
        id -> Int4,
        queue -> Varchar,
        state -> Int4,
        user_id -> Int4,
        raw -> Text,
        post_options -> Json,
        topic_id -> Nullable<Int4>,
        approved_by_id -> Nullable<Int4>,
        approved_at -> Nullable<Timestamp>,
        rejected_by_id -> Nullable<Int4>,
        rejected_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    quoted_posts (id) {
        id -> Int4,
        post_id -> Int4,
        quoted_post_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    remote_themes (id) {
        id -> Int4,
        remote_url -> Varchar,
        remote_version -> Nullable<Varchar>,
        local_version -> Nullable<Varchar>,
        about_url -> Nullable<Varchar>,
        license_url -> Nullable<Varchar>,
        commits_behind -> Nullable<Int4>,
        remote_updated_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        private_key -> Nullable<Text>,
    }
}

table! {
    scheduler_stats (id) {
        id -> Int4,
        name -> Varchar,
        hostname -> Varchar,
        pid -> Int4,
        duration_ms -> Nullable<Int4>,
        live_slots_start -> Nullable<Int4>,
        live_slots_finish -> Nullable<Int4>,
        started_at -> Timestamp,
        success -> Nullable<Bool>,
        error -> Nullable<Text>,
    }
}

table! {
    schema_migration_details (id) {
        id -> Int4,
        version -> Varchar,
        name -> Nullable<Varchar>,
        hostname -> Nullable<Varchar>,
        git_version -> Nullable<Varchar>,
        rails_version -> Nullable<Varchar>,
        duration -> Nullable<Int4>,
        direction -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

table! {
    screened_emails (id) {
        id -> Int4,
        email -> Varchar,
        action_type -> Int4,
        match_count -> Int4,
        last_match_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        ip_address -> Nullable<Inet>,
    }
}

table! {
    screened_ip_addresses (id) {
        id -> Int4,
        ip_address -> Inet,
        action_type -> Int4,
        match_count -> Int4,
        last_match_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    screened_urls (id) {
        id -> Int4,
        url -> Varchar,
        domain -> Varchar,
        action_type -> Int4,
        match_count -> Int4,
        last_match_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        ip_address -> Nullable<Inet>,
    }
}

table! {
    search_logs (id) {
        id -> Int4,
        term -> Varchar,
        user_id -> Nullable<Int4>,
        ip_address -> Inet,
        search_result_id -> Nullable<Int4>,
        search_type -> Int4,
        created_at -> Timestamp,
        search_result_type -> Nullable<Int4>,
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
    shared_drafts (id) {
        topic_id -> Int4,
        category_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        id -> Int8,
    }
}

table! {
    single_sign_on_records (id) {
        id -> Int4,
        user_id -> Int4,
        external_id -> Varchar,
        last_payload -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        external_username -> Nullable<Varchar>,
        external_email -> Nullable<Varchar>,
        external_name -> Nullable<Varchar>,
        external_avatar_url -> Nullable<Varchar>,
        external_profile_background_url -> Nullable<Varchar>,
        external_card_background_url -> Nullable<Varchar>,
    }
}

table! {
    site_settings (id) {
        id -> Int4,
        name -> Varchar,
        data_type -> Int4,
        value -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_addresses (id) {
        id -> Int4,
        firstname -> Nullable<Varchar>,
        lastname -> Nullable<Varchar>,
        address1 -> Nullable<Varchar>,
        address2 -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        zipcode -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        state_name -> Nullable<Varchar>,
        alternative_phone -> Nullable<Varchar>,
        company -> Nullable<Varchar>,
        state_id -> Nullable<Int4>,
        country_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_adjustment_reasons (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        code -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_adjustments (id) {
        id -> Int4,
        source_type -> Nullable<Varchar>,
        source_id -> Nullable<Int4>,
        adjustable_type -> Nullable<Varchar>,
        adjustable_id -> Int4,
        amount -> Nullable<Numeric>,
        label -> Nullable<Varchar>,
        eligible -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        order_id -> Int4,
        included -> Nullable<Bool>,
        promotion_code_id -> Nullable<Int4>,
        adjustment_reason_id -> Nullable<Int4>,
        finalized -> Bool,
    }
}

table! {
    spree_assets (id) {
        id -> Int4,
        viewable_type -> Nullable<Varchar>,
        viewable_id -> Nullable<Int4>,
        attachment_width -> Nullable<Int4>,
        attachment_height -> Nullable<Int4>,
        attachment_file_size -> Nullable<Int4>,
        position -> Nullable<Int4>,
        attachment_content_type -> Nullable<Varchar>,
        attachment_file_name -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        attachment_updated_at -> Nullable<Timestamp>,
        alt -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_calculators (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        calculable_type -> Nullable<Varchar>,
        calculable_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        preferences -> Nullable<Text>,
    }
}

table! {
    spree_cartons (id) {
        id -> Int4,
        number -> Nullable<Varchar>,
        external_number -> Nullable<Varchar>,
        stock_location_id -> Nullable<Int4>,
        address_id -> Nullable<Int4>,
        shipping_method_id -> Nullable<Int4>,
        tracking -> Nullable<Varchar>,
        shipped_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        imported_from_shipment_id -> Nullable<Int4>,
    }
}

table! {
    spree_comments (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        comment -> Nullable<Text>,
        commentable_type -> Nullable<Varchar>,
        commentable_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        comment_type_id -> Nullable<Int4>,
    }
}

table! {
    spree_comment_types (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        applies_to -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_countries (id) {
        id -> Int4,
        iso_name -> Nullable<Varchar>,
        iso -> Nullable<Varchar>,
        iso3 -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        numcode -> Nullable<Int4>,
        states_required -> Nullable<Bool>,
        updated_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_credit_cards (id) {
        id -> Int4,
        month -> Nullable<Varchar>,
        year -> Nullable<Varchar>,
        cc_type -> Nullable<Varchar>,
        last_digits -> Nullable<Varchar>,
        gateway_customer_profile_id -> Nullable<Varchar>,
        gateway_payment_profile_id -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        name -> Nullable<Varchar>,
        user_id -> Nullable<Int4>,
        payment_method_id -> Nullable<Int4>,
        default -> Bool,
        address_id -> Nullable<Int4>,
    }
}

table! {
    spree_customer_returns (id) {
        id -> Int4,
        number -> Nullable<Varchar>,
        stock_location_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_inventory_units (id) {
        id -> Int4,
        state -> Nullable<Varchar>,
        variant_id -> Nullable<Int4>,
        shipment_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        pending -> Nullable<Bool>,
        line_item_id -> Nullable<Int4>,
        carton_id -> Nullable<Int4>,
    }
}

table! {
    spree_line_item_actions (id) {
        id -> Int4,
        line_item_id -> Int4,
        action_id -> Int4,
        quantity -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_line_items (id) {
        id -> Int4,
        variant_id -> Nullable<Int4>,
        order_id -> Nullable<Int4>,
        quantity -> Int4,
        price -> Numeric,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        cost_price -> Nullable<Numeric>,
        tax_category_id -> Nullable<Int4>,
        adjustment_total -> Nullable<Numeric>,
        additional_tax_total -> Nullable<Numeric>,
        promo_total -> Nullable<Numeric>,
        included_tax_total -> Numeric,
    }
}

table! {
    spree_log_entries (id) {
        id -> Int4,
        source_type -> Nullable<Varchar>,
        source_id -> Nullable<Int4>,
        details -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_option_type_prototypes (id) {
        id -> Int4,
        prototype_id -> Nullable<Int4>,
        option_type_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_option_types (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        presentation -> Nullable<Varchar>,
        position -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_option_values (id) {
        id -> Int4,
        position -> Nullable<Int4>,
        name -> Nullable<Varchar>,
        presentation -> Nullable<Varchar>,
        option_type_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_option_values_variants (id) {
        id -> Int4,
        variant_id -> Nullable<Int4>,
        option_value_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_order_mutexes (id) {
        id -> Int4,
        order_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_orders (id) {
        id -> Int4,
        number -> Nullable<Varchar>,
        item_total -> Numeric,
        total -> Numeric,
        state -> Nullable<Varchar>,
        adjustment_total -> Numeric,
        user_id -> Nullable<Int4>,
        completed_at -> Nullable<Timestamp>,
        bill_address_id -> Nullable<Int4>,
        ship_address_id -> Nullable<Int4>,
        payment_total -> Nullable<Numeric>,
        shipment_state -> Nullable<Varchar>,
        payment_state -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        special_instructions -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        currency -> Nullable<Varchar>,
        last_ip_address -> Nullable<Varchar>,
        created_by_id -> Nullable<Int4>,
        shipment_total -> Numeric,
        additional_tax_total -> Nullable<Numeric>,
        promo_total -> Nullable<Numeric>,
        channel -> Nullable<Varchar>,
        included_tax_total -> Numeric,
        item_count -> Nullable<Int4>,
        approver_id -> Nullable<Int4>,
        approved_at -> Nullable<Timestamp>,
        confirmation_delivered -> Nullable<Bool>,
        guest_token -> Nullable<Varchar>,
        canceled_at -> Nullable<Timestamp>,
        canceler_id -> Nullable<Int4>,
        store_id -> Nullable<Int4>,
        approver_name -> Nullable<Varchar>,
        frontend_viewable -> Bool,
        invoice_number -> Nullable<Int4>,
        invoice_date -> Nullable<Date>,
    }
}

table! {
    spree_orders_promotions (id) {
        id -> Int4,
        order_id -> Nullable<Int4>,
        promotion_id -> Nullable<Int4>,
        promotion_code_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_payment_capture_events (id) {
        id -> Int4,
        amount -> Nullable<Numeric>,
        payment_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_payment_methods (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        active -> Nullable<Bool>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        auto_capture -> Nullable<Bool>,
        preferences -> Nullable<Text>,
        preference_source -> Nullable<Varchar>,
        position -> Nullable<Int4>,
        available_to_users -> Nullable<Bool>,
        available_to_admin -> Nullable<Bool>,
    }
}

table! {
    spree_payments (id) {
        id -> Int4,
        amount -> Numeric,
        order_id -> Nullable<Int4>,
        source_type -> Nullable<Varchar>,
        source_id -> Nullable<Int4>,
        payment_method_id -> Nullable<Int4>,
        state -> Nullable<Varchar>,
        response_code -> Nullable<Varchar>,
        avs_response -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        number -> Nullable<Varchar>,
        cvv_response_code -> Nullable<Varchar>,
        cvv_response_message -> Nullable<Varchar>,
    }
}

table! {
    spree_preferences (id) {
        id -> Int4,
        value -> Nullable<Text>,
        key -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_prices (id) {
        id -> Int4,
        variant_id -> Int4,
        amount -> Nullable<Numeric>,
        currency -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        country_iso -> Nullable<Varchar>,
    }
}

table! {
    spree_product_option_types (id) {
        id -> Int4,
        position -> Nullable<Int4>,
        product_id -> Nullable<Int4>,
        option_type_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_product_promotion_rules (id) {
        id -> Int4,
        product_id -> Nullable<Int4>,
        promotion_rule_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_product_properties (id) {
        id -> Int4,
        value -> Nullable<Varchar>,
        product_id -> Nullable<Int4>,
        property_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        position -> Nullable<Int4>,
    }
}

table! {
    spree_products (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        available_on -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        slug -> Nullable<Varchar>,
        meta_description -> Nullable<Text>,
        meta_keywords -> Nullable<Varchar>,
        tax_category_id -> Nullable<Int4>,
        shipping_category_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        promotionable -> Nullable<Bool>,
        meta_title -> Nullable<Varchar>,
    }
}

table! {
    spree_products_taxons (id) {
        id -> Int4,
        product_id -> Nullable<Int4>,
        taxon_id -> Nullable<Int4>,
        position -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_promotion_action_line_items (id) {
        id -> Int4,
        promotion_action_id -> Nullable<Int4>,
        variant_id -> Nullable<Int4>,
        quantity -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_promotion_actions (id) {
        id -> Int4,
        promotion_id -> Nullable<Int4>,
        position -> Nullable<Int4>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
        preferences -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_promotion_categories (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        code -> Nullable<Varchar>,
    }
}

table! {
    spree_promotion_code_batches (id) {
        id -> Int4,
        promotion_id -> Int4,
        base_code -> Varchar,
        number_of_codes -> Int4,
        email -> Nullable<Varchar>,
        error -> Nullable<Varchar>,
        state -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        join_characters -> Varchar,
    }
}

table! {
    spree_promotion_codes (id) {
        id -> Int4,
        promotion_id -> Int4,
        value -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        promotion_code_batch_id -> Nullable<Int4>,
    }
}

table! {
    spree_promotion_rules (id) {
        id -> Int4,
        promotion_id -> Nullable<Int4>,
        product_group_id -> Nullable<Int4>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        code -> Nullable<Varchar>,
        preferences -> Nullable<Text>,
    }
}

table! {
    spree_promotion_rules_stores (id) {
        id -> Int8,
        store_id -> Int8,
        promotion_rule_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_promotion_rules_users (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        promotion_rule_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_promotion_rule_taxons (id) {
        id -> Int4,
        taxon_id -> Nullable<Int4>,
        promotion_rule_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_promotions (id) {
        id -> Int4,
        description -> Nullable<Varchar>,
        expires_at -> Nullable<Timestamp>,
        starts_at -> Nullable<Timestamp>,
        name -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        usage_limit -> Nullable<Int4>,
        match_policy -> Nullable<Varchar>,
        code -> Nullable<Varchar>,
        advertise -> Nullable<Bool>,
        path -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        promotion_category_id -> Nullable<Int4>,
        per_code_usage_limit -> Nullable<Int4>,
        apply_automatically -> Nullable<Bool>,
    }
}

table! {
    spree_properties (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        presentation -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_property_prototypes (id) {
        id -> Int4,
        prototype_id -> Nullable<Int4>,
        property_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_prototypes (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_prototype_taxons (id) {
        id -> Int4,
        taxon_id -> Nullable<Int4>,
        prototype_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_refund_reasons (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        mutable -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        code -> Nullable<Varchar>,
    }
}

table! {
    spree_refunds (id) {
        id -> Int4,
        payment_id -> Nullable<Int4>,
        amount -> Numeric,
        transaction_id -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        refund_reason_id -> Nullable<Int4>,
        reimbursement_id -> Nullable<Int4>,
    }
}

table! {
    spree_reimbursement_credits (id) {
        id -> Int4,
        amount -> Numeric,
        reimbursement_id -> Nullable<Int4>,
        creditable_id -> Nullable<Int4>,
        creditable_type -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_reimbursements (id) {
        id -> Int4,
        number -> Nullable<Varchar>,
        reimbursement_status -> Nullable<Varchar>,
        customer_return_id -> Nullable<Int4>,
        order_id -> Nullable<Int4>,
        total -> Nullable<Numeric>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_reimbursement_types (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        mutable -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
    }
}

table! {
    spree_relations (id) {
        id -> Int4,
        relation_type_id -> Nullable<Int4>,
        relatable_type -> Nullable<Varchar>,
        relatable_id -> Nullable<Int4>,
        related_to_type -> Nullable<Varchar>,
        related_to_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        discount_amount -> Nullable<Numeric>,
        position -> Nullable<Int4>,
    }
}

table! {
    spree_relation_types (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        applies_to -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_return_authorizations (id) {
        id -> Int4,
        number -> Nullable<Varchar>,
        state -> Nullable<Varchar>,
        order_id -> Nullable<Int4>,
        memo -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        stock_location_id -> Nullable<Int4>,
        return_reason_id -> Nullable<Int4>,
    }
}

table! {
    spree_return_items (id) {
        id -> Int4,
        return_authorization_id -> Nullable<Int4>,
        inventory_unit_id -> Nullable<Int4>,
        exchange_variant_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        amount -> Numeric,
        included_tax_total -> Numeric,
        additional_tax_total -> Numeric,
        reception_status -> Nullable<Varchar>,
        acceptance_status -> Nullable<Varchar>,
        customer_return_id -> Nullable<Int4>,
        reimbursement_id -> Nullable<Int4>,
        exchange_inventory_unit_id -> Nullable<Int4>,
        acceptance_status_errors -> Nullable<Text>,
        preferred_reimbursement_type_id -> Nullable<Int4>,
        override_reimbursement_type_id -> Nullable<Int4>,
        resellable -> Bool,
        return_reason_id -> Nullable<Int4>,
    }
}

table! {
    spree_return_reasons (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        mutable -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_roles (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_roles_users (id) {
        id -> Int4,
        role_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_shipments (id) {
        id -> Int4,
        tracking -> Nullable<Varchar>,
        number -> Nullable<Varchar>,
        cost -> Nullable<Numeric>,
        shipped_at -> Nullable<Timestamp>,
        order_id -> Nullable<Int4>,
        deprecated_address_id -> Nullable<Int4>,
        state -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        stock_location_id -> Nullable<Int4>,
        adjustment_total -> Nullable<Numeric>,
        additional_tax_total -> Nullable<Numeric>,
        promo_total -> Nullable<Numeric>,
        included_tax_total -> Numeric,
    }
}

table! {
    spree_shipping_categories (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_shipping_method_categories (id) {
        id -> Int4,
        shipping_method_id -> Int4,
        shipping_category_id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_shipping_methods (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        tracking_url -> Nullable<Varchar>,
        admin_name -> Nullable<Varchar>,
        tax_category_id -> Nullable<Int4>,
        code -> Nullable<Varchar>,
        available_to_all -> Nullable<Bool>,
        carrier -> Nullable<Varchar>,
        service_level -> Nullable<Varchar>,
        available_to_users -> Nullable<Bool>,
    }
}

table! {
    spree_shipping_method_stock_locations (id) {
        id -> Int4,
        shipping_method_id -> Nullable<Int4>,
        stock_location_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_shipping_method_zones (id) {
        id -> Int4,
        shipping_method_id -> Nullable<Int4>,
        zone_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_shipping_rates (id) {
        id -> Int4,
        shipment_id -> Nullable<Int4>,
        shipping_method_id -> Nullable<Int4>,
        selected -> Nullable<Bool>,
        cost -> Nullable<Numeric>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        tax_rate_id -> Nullable<Int4>,
    }
}

table! {
    spree_shipping_rate_taxes (id) {
        id -> Int4,
        amount -> Numeric,
        tax_rate_id -> Nullable<Int4>,
        shipping_rate_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_state_changes (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        previous_state -> Nullable<Varchar>,
        stateful_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        stateful_type -> Nullable<Varchar>,
        next_state -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_states (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        abbr -> Nullable<Varchar>,
        country_id -> Nullable<Int4>,
        updated_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_stock_items (id) {
        id -> Int4,
        stock_location_id -> Nullable<Int4>,
        variant_id -> Nullable<Int4>,
        count_on_hand -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        backorderable -> Nullable<Bool>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_stock_locations (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        default -> Bool,
        address1 -> Nullable<Varchar>,
        address2 -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        state_id -> Nullable<Int4>,
        state_name -> Nullable<Varchar>,
        country_id -> Nullable<Int4>,
        zipcode -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        backorderable_default -> Nullable<Bool>,
        propagate_all_variants -> Nullable<Bool>,
        admin_name -> Nullable<Varchar>,
        position -> Nullable<Int4>,
        restock_inventory -> Bool,
        fulfillable -> Bool,
        code -> Nullable<Varchar>,
        check_stock_on_transfer -> Nullable<Bool>,
    }
}

table! {
    spree_stock_movements (id) {
        id -> Int4,
        stock_item_id -> Nullable<Int4>,
        quantity -> Nullable<Int4>,
        action -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        originator_type -> Nullable<Varchar>,
        originator_id -> Nullable<Int4>,
    }
}

table! {
    spree_store_credit_categories (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_store_credit_events (id) {
        id -> Int4,
        store_credit_id -> Int4,
        action -> Varchar,
        amount -> Nullable<Numeric>,
        user_total_amount -> Numeric,
        authorization_code -> Varchar,
        deleted_at -> Nullable<Timestamp>,
        originator_type -> Nullable<Varchar>,
        originator_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        update_reason_id -> Nullable<Int4>,
        amount_remaining -> Nullable<Numeric>,
    }
}

table! {
    spree_store_credits (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        category_id -> Nullable<Int4>,
        created_by_id -> Nullable<Int4>,
        amount -> Numeric,
        amount_used -> Numeric,
        amount_authorized -> Numeric,
        currency -> Nullable<Varchar>,
        memo -> Nullable<Text>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        type_id -> Nullable<Int4>,
        invalidated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_store_credit_types (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        priority -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_store_credit_update_reasons (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_store_payment_methods (id) {
        id -> Int4,
        store_id -> Int4,
        payment_method_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_stores (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        url -> Nullable<Varchar>,
        meta_description -> Nullable<Text>,
        meta_keywords -> Nullable<Text>,
        seo_title -> Nullable<Varchar>,
        mail_from_address -> Nullable<Varchar>,
        default_currency -> Nullable<Varchar>,
        code -> Nullable<Varchar>,
        default -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        cart_tax_country_iso -> Nullable<Varchar>,
        available_locales -> Nullable<Varchar>,
    }
}

table! {
    spree_store_shipping_methods (id) {
        id -> Int8,
        store_id -> Int8,
        shipping_method_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_tax_categories (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        is_default -> Nullable<Bool>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        tax_code -> Nullable<Varchar>,
    }
}

table! {
    spree_taxonomies (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        position -> Nullable<Int4>,
    }
}

table! {
    spree_taxons (id) {
        id -> Int4,
        parent_id -> Nullable<Int4>,
        position -> Nullable<Int4>,
        name -> Varchar,
        permalink -> Nullable<Varchar>,
        taxonomy_id -> Nullable<Int4>,
        lft -> Nullable<Int4>,
        rgt -> Nullable<Int4>,
        icon_file_name -> Nullable<Varchar>,
        icon_content_type -> Nullable<Varchar>,
        icon_file_size -> Nullable<Int4>,
        icon_updated_at -> Nullable<Timestamp>,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        meta_title -> Nullable<Varchar>,
        meta_description -> Nullable<Varchar>,
        meta_keywords -> Nullable<Varchar>,
        depth -> Nullable<Int4>,
    }
}

table! {
    spree_tax_rates (id) {
        id -> Int4,
        amount -> Nullable<Numeric>,
        zone_id -> Nullable<Int4>,
        included_in_price -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        name -> Nullable<Varchar>,
        show_rate_in_label -> Nullable<Bool>,
        deleted_at -> Nullable<Timestamp>,
        starts_at -> Nullable<Timestamp>,
        expires_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_tax_rate_tax_categories (id) {
        id -> Int4,
        tax_category_id -> Int4,
        tax_rate_id -> Int4,
    }
}

table! {
    spree_unit_cancels (id) {
        id -> Int4,
        inventory_unit_id -> Int4,
        reason -> Nullable<Varchar>,
        created_by -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_user_addresses (id) {
        id -> Int4,
        user_id -> Int4,
        address_id -> Int4,
        default -> Nullable<Bool>,
        archived -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_users (id) {
        id -> Int4,
        encrypted_password -> Nullable<Varchar>,
        password_salt -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        remember_token -> Nullable<Varchar>,
        persistence_token -> Nullable<Varchar>,
        reset_password_token -> Nullable<Varchar>,
        perishable_token -> Nullable<Varchar>,
        sign_in_count -> Int4,
        failed_attempts -> Int4,
        last_request_at -> Nullable<Timestamp>,
        current_sign_in_at -> Nullable<Timestamp>,
        last_sign_in_at -> Nullable<Timestamp>,
        current_sign_in_ip -> Nullable<Varchar>,
        last_sign_in_ip -> Nullable<Varchar>,
        login -> Nullable<Varchar>,
        ship_address_id -> Nullable<Int4>,
        bill_address_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        spree_api_key -> Nullable<Varchar>,
        authentication_token -> Nullable<Varchar>,
        unlock_token -> Nullable<Varchar>,
        locked_at -> Nullable<Timestamp>,
        remember_created_at -> Nullable<Timestamp>,
        reset_password_sent_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        confirmation_token -> Nullable<Varchar>,
        confirmed_at -> Nullable<Timestamp>,
        confirmation_sent_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_user_stock_locations (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        stock_location_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_variant_property_rule_conditions (id) {
        id -> Int4,
        option_value_id -> Nullable<Int4>,
        variant_property_rule_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_variant_property_rules (id) {
        id -> Int4,
        product_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_variant_property_rule_values (id) {
        id -> Int4,
        value -> Nullable<Text>,
        position -> Nullable<Int4>,
        property_id -> Nullable<Int4>,
        variant_property_rule_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_variants (id) {
        id -> Int4,
        sku -> Varchar,
        weight -> Nullable<Numeric>,
        height -> Nullable<Numeric>,
        width -> Nullable<Numeric>,
        depth -> Nullable<Numeric>,
        deleted_at -> Nullable<Timestamp>,
        is_master -> Nullable<Bool>,
        product_id -> Nullable<Int4>,
        cost_price -> Nullable<Numeric>,
        position -> Nullable<Int4>,
        cost_currency -> Nullable<Varchar>,
        track_inventory -> Nullable<Bool>,
        tax_category_id -> Nullable<Int4>,
        updated_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_wallet_payment_sources (id) {
        id -> Int4,
        user_id -> Int4,
        payment_source_type -> Varchar,
        payment_source_id -> Int4,
        default -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_wished_products (id) {
        id -> Int4,
        variant_id -> Nullable<Int4>,
        wishlist_id -> Nullable<Int4>,
        remark -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        quantity -> Int4,
    }
}

table! {
    spree_wishlists (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        name -> Nullable<Varchar>,
        access_hash -> Nullable<Varchar>,
        is_private -> Bool,
        is_default -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_zone_members (id) {
        id -> Int4,
        zoneable_type -> Nullable<Varchar>,
        zoneable_id -> Nullable<Int4>,
        zone_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_zones (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        zone_members_count -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    stylesheet_cache (id) {
        id -> Int4,
        target -> Varchar,
        digest -> Varchar,
        content -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        theme_id -> Int4,
        source_map -> Nullable<Text>,
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
    tag_group_memberships (id) {
        id -> Int4,
        tag_id -> Int4,
        tag_group_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tag_group_permissions (id) {
        id -> Int8,
        tag_group_id -> Int8,
        group_id -> Int8,
        permission_type -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tag_groups (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        parent_tag_id -> Nullable<Int4>,
        one_per_topic -> Nullable<Bool>,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
        topic_count -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        pm_topic_count -> Int4,
    }
}

table! {
    tag_search_data (tag_id) {
        tag_id -> Int4,
        search_data -> Nullable<Tsvector>,
        raw_data -> Nullable<Text>,
        locale -> Nullable<Text>,
        version -> Nullable<Int4>,
    }
}

table! {
    tag_users (id) {
        id -> Int4,
        tag_id -> Int4,
        user_id -> Int4,
        notification_level -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    theme_fields (id) {
        id -> Int4,
        theme_id -> Int4,
        target_id -> Int4,
        name -> Varchar,
        value -> Text,
        value_baked -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        compiler_version -> Int4,
        error -> Nullable<Varchar>,
        upload_id -> Nullable<Int4>,
        type_id -> Int4,
    }
}

table! {
    themes (id) {
        id -> Int4,
        name -> Varchar,
        user_id -> Int4,
        key -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        compiler_version -> Int4,
        user_selectable -> Bool,
        hidden -> Bool,
        color_scheme_id -> Nullable<Int4>,
        remote_theme_id -> Nullable<Int4>,
    }
}

table! {
    theme_settings (id) {
        id -> Int8,
        name -> Varchar,
        data_type -> Int4,
        value -> Nullable<Text>,
        theme_id -> Int4,
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
    topic_allowed_groups (id) {
        id -> Int4,
        group_id -> Int4,
        topic_id -> Int4,
    }
}

table! {
    topic_allowed_users (id) {
        id -> Int4,
        user_id -> Int4,
        topic_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    topic_custom_fields (id) {
        id -> Int4,
        topic_id -> Int4,
        name -> Varchar,
        value -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    topic_embeds (id) {
        id -> Int4,
        topic_id -> Int4,
        post_id -> Int4,
        embed_url -> Varchar,
        content_sha1 -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        deleted_by_id -> Nullable<Int4>,
    }
}

table! {
    topic_invites (id) {
        id -> Int4,
        topic_id -> Int4,
        invite_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    topic_link_clicks (id) {
        id -> Int4,
        topic_link_id -> Int4,
        user_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        ip_address -> Inet,
    }
}

table! {
    topic_links (id) {
        id -> Int4,
        topic_id -> Int4,
        post_id -> Nullable<Int4>,
        user_id -> Int4,
        url -> Varchar,
        domain -> Varchar,
        internal -> Bool,
        link_topic_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        reflection -> Nullable<Bool>,
        clicks -> Int4,
        link_post_id -> Nullable<Int4>,
        title -> Nullable<Varchar>,
        crawled_at -> Nullable<Timestamp>,
        quote -> Bool,
        extension -> Nullable<Varchar>,
    }
}

table! {
    topics (id) {
        id -> Int4,
        title -> Varchar,
        last_posted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        views -> Int4,
        posts_count -> Int4,
        user_id -> Nullable<Int4>,
        last_post_user_id -> Int4,
        reply_count -> Int4,
        featured_user1_id -> Nullable<Int4>,
        featured_user2_id -> Nullable<Int4>,
        featured_user3_id -> Nullable<Int4>,
        avg_time -> Nullable<Int4>,
        deleted_at -> Nullable<Timestamp>,
        highest_post_number -> Int4,
        image_url -> Nullable<Varchar>,
        like_count -> Int4,
        incoming_link_count -> Int4,
        category_id -> Nullable<Int4>,
        visible -> Bool,
        moderator_posts_count -> Int4,
        closed -> Bool,
        archived -> Bool,
        bumped_at -> Timestamp,
        has_summary -> Bool,
        vote_count -> Int4,
        archetype -> Varchar,
        featured_user4_id -> Nullable<Int4>,
        notify_moderators_count -> Int4,
        spam_count -> Int4,
        pinned_at -> Nullable<Timestamp>,
        score -> Nullable<Float8>,
        percent_rank -> Float8,
        subtype -> Nullable<Varchar>,
        slug -> Nullable<Varchar>,
        deleted_by_id -> Nullable<Int4>,
        participant_count -> Nullable<Int4>,
        word_count -> Nullable<Int4>,
        excerpt -> Nullable<Varchar>,
        pinned_globally -> Bool,
        pinned_until -> Nullable<Timestamp>,
        fancy_title -> Nullable<Varchar>,
        highest_staff_post_number -> Int4,
        featured_link -> Nullable<Varchar>,
    }
}

table! {
    topic_search_data (topic_id) {
        topic_id -> Int4,
        raw_data -> Nullable<Text>,
        locale -> Varchar,
        search_data -> Nullable<Tsvector>,
        version -> Nullable<Int4>,
    }
}

table! {
    topic_tags (id) {
        id -> Int4,
        topic_id -> Int4,
        tag_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    topic_timers (id) {
        id -> Int4,
        execute_at -> Timestamp,
        status_type -> Int4,
        user_id -> Int4,
        topic_id -> Int4,
        based_on_last_post -> Bool,
        deleted_at -> Nullable<Timestamp>,
        deleted_by_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        category_id -> Nullable<Int4>,
        public_type -> Nullable<Bool>,
    }
}

table! {
    topic_users (id) {
        user_id -> Int4,
        topic_id -> Int4,
        posted -> Bool,
        last_read_post_number -> Nullable<Int4>,
        highest_seen_post_number -> Nullable<Int4>,
        last_visited_at -> Nullable<Timestamp>,
        first_visited_at -> Nullable<Timestamp>,
        notification_level -> Int4,
        notifications_changed_at -> Nullable<Timestamp>,
        notifications_reason_id -> Nullable<Int4>,
        total_msecs_viewed -> Int4,
        cleared_pinned_at -> Nullable<Timestamp>,
        id -> Int8,
        last_emailed_post_number -> Nullable<Int4>,
        liked -> Nullable<Bool>,
        bookmarked -> Nullable<Bool>,
    }
}

table! {
    topic_views (id) {
        id -> Int4,
        topic_id -> Int4,
        viewed_at -> Date,
        user_id -> Nullable<Int4>,
        ip_address -> Inet,
    }
}

table! {
    top_topics (id) {
        id -> Int4,
        topic_id -> Nullable<Int4>,
        yearly_posts_count -> Int4,
        yearly_views_count -> Int4,
        yearly_likes_count -> Int4,
        monthly_posts_count -> Int4,
        monthly_views_count -> Int4,
        monthly_likes_count -> Int4,
        weekly_posts_count -> Int4,
        weekly_views_count -> Int4,
        weekly_likes_count -> Int4,
        daily_posts_count -> Int4,
        daily_views_count -> Int4,
        daily_likes_count -> Int4,
        daily_score -> Nullable<Float8>,
        weekly_score -> Nullable<Float8>,
        monthly_score -> Nullable<Float8>,
        yearly_score -> Nullable<Float8>,
        all_score -> Nullable<Float8>,
        daily_op_likes_count -> Int4,
        weekly_op_likes_count -> Int4,
        monthly_op_likes_count -> Int4,
        yearly_op_likes_count -> Int4,
        quarterly_posts_count -> Int4,
        quarterly_views_count -> Int4,
        quarterly_likes_count -> Int4,
        quarterly_score -> Nullable<Float8>,
        quarterly_op_likes_count -> Int4,
    }
}

table! {
    translation_overrides (id) {
        id -> Int4,
        locale -> Varchar,
        translation_key -> Varchar,
        value -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        compiled_js -> Nullable<Text>,
    }
}

table! {
    twitter_user_infos (id) {
        id -> Int4,
        user_id -> Int4,
        screen_name -> Varchar,
        twitter_user_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        email -> Nullable<Varchar>,
    }
}

table! {
    unsubscribe_keys (key) {
        key -> Varchar,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        unsubscribe_key_type -> Nullable<Varchar>,
        topic_id -> Nullable<Int4>,
        post_id -> Nullable<Int4>,
    }
}

table! {
    uploads (id) {
        id -> Int4,
        user_id -> Int4,
        original_filename -> Varchar,
        filesize -> Int4,
        width -> Nullable<Int4>,
        height -> Nullable<Int4>,
        url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        sha1 -> Nullable<Varchar>,
        origin -> Nullable<Varchar>,
        retain_hours -> Nullable<Int4>,
        extension -> Nullable<Varchar>,
    }
}

table! {
    user_actions (id) {
        id -> Int4,
        action_type -> Int4,
        user_id -> Int4,
        target_topic_id -> Nullable<Int4>,
        target_post_id -> Nullable<Int4>,
        target_user_id -> Nullable<Int4>,
        acting_user_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        queued_post_id -> Nullable<Int4>,
    }
}

table! {
    user_api_keys (id) {
        id -> Int4,
        user_id -> Int4,
        client_id -> Varchar,
        key -> Varchar,
        application_name -> Varchar,
        push_url -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        revoked_at -> Nullable<Timestamp>,
        scopes -> Array<Text>,
    }
}

table! {
    user_archived_messages (id) {
        id -> Int4,
        user_id -> Int4,
        topic_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_auth_token_logs (id) {
        id -> Int4,
        action -> Varchar,
        user_auth_token_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        client_ip -> Nullable<Inet>,
        user_agent -> Nullable<Varchar>,
        auth_token -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        path -> Nullable<Varchar>,
    }
}

table! {
    user_auth_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        auth_token -> Varchar,
        prev_auth_token -> Varchar,
        user_agent -> Nullable<Varchar>,
        auth_token_seen -> Bool,
        client_ip -> Nullable<Inet>,
        rotated_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        seen_at -> Nullable<Timestamp>,
    }
}

table! {
    user_avatars (id) {
        id -> Int4,
        user_id -> Int4,
        custom_upload_id -> Nullable<Int4>,
        gravatar_upload_id -> Nullable<Int4>,
        last_gravatar_download_attempt -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_badges (id) {
        id -> Int4,
        badge_id -> Int4,
        user_id -> Int4,
        granted_at -> Timestamp,
        granted_by_id -> Int4,
        post_id -> Nullable<Int4>,
        notification_id -> Nullable<Int4>,
        seq -> Int4,
    }
}

table! {
    user_custom_fields (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        value -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_emails (id) {
        id -> Int4,
        user_id -> Int4,
        email -> Varchar,
        primary -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_exports (id) {
        id -> Int4,
        file_name -> Varchar,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        upload_id -> Nullable<Int4>,
    }
}

table! {
    user_field_options (id) {
        id -> Int4,
        user_field_id -> Int4,
        value -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_fields (id) {
        id -> Int4,
        name -> Varchar,
        field_type -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        editable -> Bool,
        description -> Varchar,
        required -> Bool,
        show_on_profile -> Bool,
        position -> Nullable<Int4>,
        show_on_user_card -> Bool,
    }
}

table! {
    user_histories (id) {
        id -> Int4,
        action -> Int4,
        acting_user_id -> Nullable<Int4>,
        target_user_id -> Nullable<Int4>,
        details -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        context -> Nullable<Varchar>,
        ip_address -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        subject -> Nullable<Text>,
        previous_value -> Nullable<Text>,
        new_value -> Nullable<Text>,
        topic_id -> Nullable<Int4>,
        admin_only -> Nullable<Bool>,
        post_id -> Nullable<Int4>,
        custom_type -> Nullable<Varchar>,
        category_id -> Nullable<Int4>,
    }
}

table! {
    user_open_ids (id) {
        id -> Int4,
        user_id -> Int4,
        email -> Varchar,
        url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        active -> Bool,
    }
}

table! {
    user_options (id) {
        id -> Int4,
        user_id -> Int4,
        email_always -> Bool,
        mailing_list_mode -> Bool,
        email_digests -> Nullable<Bool>,
        email_direct -> Bool,
        email_private_messages -> Bool,
        external_links_in_new_tab -> Bool,
        enable_quoting -> Bool,
        dynamic_favicon -> Bool,
        disable_jump_reply -> Bool,
        automatically_unpin_topics -> Bool,
        digest_after_minutes -> Nullable<Int4>,
        auto_track_topics_after_msecs -> Nullable<Int4>,
        new_topic_duration_minutes -> Nullable<Int4>,
        last_redirected_to_top_at -> Nullable<Timestamp>,
        email_previous_replies -> Int4,
        email_in_reply_to -> Bool,
        like_notification_frequency -> Int4,
        mailing_list_mode_frequency -> Int4,
        include_tl0_in_digests -> Nullable<Bool>,
        notification_level_when_replying -> Nullable<Int4>,
        theme_key -> Nullable<Varchar>,
        theme_key_seq -> Int4,
        allow_private_messages -> Bool,
        homepage_id -> Nullable<Int4>,
    }
}

table! {
    user_profiles (user_id) {
        user_id -> Int4,
        location -> Nullable<Varchar>,
        website -> Nullable<Varchar>,
        bio_raw -> Nullable<Text>,
        bio_cooked -> Nullable<Text>,
        profile_background -> Nullable<Varchar>,
        dismissed_banner_key -> Nullable<Int4>,
        bio_cooked_version -> Nullable<Int4>,
        badge_granted_title -> Nullable<Bool>,
        card_background -> Nullable<Varchar>,
        views -> Int4,
    }
}

table! {
    user_profile_views (id) {
        id -> Int4,
        user_profile_id -> Int4,
        viewed_at -> Timestamp,
        ip_address -> Inet,
        user_id -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name -> Nullable<Varchar>,
        seen_notification_id -> Int4,
        last_posted_at -> Nullable<Timestamp>,
        password_hash -> Nullable<Varchar>,
        salt -> Nullable<Varchar>,
        active -> Bool,
        username_lower -> Varchar,
        last_seen_at -> Nullable<Timestamp>,
        admin -> Bool,
        last_emailed_at -> Nullable<Timestamp>,
        trust_level -> Int4,
        approved -> Bool,
        approved_by_id -> Nullable<Int4>,
        approved_at -> Nullable<Timestamp>,
        previous_visit_at -> Nullable<Timestamp>,
        suspended_at -> Nullable<Timestamp>,
        suspended_till -> Nullable<Timestamp>,
        date_of_birth -> Nullable<Date>,
        views -> Int4,
        flag_level -> Int4,
        ip_address -> Nullable<Inet>,
        moderator -> Nullable<Bool>,
        title -> Nullable<Varchar>,
        uploaded_avatar_id -> Nullable<Int4>,
        locale -> Nullable<Varchar>,
        primary_group_id -> Nullable<Int4>,
        registration_ip_address -> Nullable<Inet>,
        staged -> Bool,
        first_seen_at -> Nullable<Timestamp>,
        silenced_till -> Nullable<Timestamp>,
        group_locked_trust_level -> Nullable<Int4>,
        manual_locked_trust_level -> Nullable<Int4>,
    }
}

table! {
    user_search_data (user_id) {
        user_id -> Int4,
        search_data -> Nullable<Tsvector>,
        raw_data -> Nullable<Text>,
        locale -> Nullable<Text>,
        version -> Nullable<Int4>,
    }
}

table! {
    user_second_factors (id) {
        id -> Int8,
        user_id -> Int4,
        method -> Int4,
        data -> Varchar,
        enabled -> Bool,
        last_used -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_stats (user_id) {
        user_id -> Int4,
        topics_entered -> Int4,
        time_read -> Int4,
        days_visited -> Int4,
        posts_read_count -> Int4,
        likes_given -> Int4,
        likes_received -> Int4,
        topic_reply_count -> Int4,
        new_since -> Timestamp,
        read_faq -> Nullable<Timestamp>,
        first_post_created_at -> Nullable<Timestamp>,
        post_count -> Int4,
        topic_count -> Int4,
        bounce_score -> Int4,
        reset_bounce_score_after -> Nullable<Timestamp>,
    }
}

table! {
    user_visits (id) {
        id -> Int4,
        user_id -> Int4,
        visited_at -> Date,
        posts_read -> Nullable<Int4>,
        mobile -> Nullable<Bool>,
        time_read -> Int4,
    }
}

table! {
    user_warnings (id) {
        id -> Int4,
        topic_id -> Int4,
        user_id -> Int4,
        created_by_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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

table! {
    watched_words (id) {
        id -> Int4,
        word -> Varchar,
        action -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    web_crawler_requests (id) {
        id -> Int8,
        date -> Date,
        user_agent -> Varchar,
        count -> Int4,
    }
}

table! {
    web_hook_events (id) {
        id -> Int4,
        web_hook_id -> Int4,
        headers -> Nullable<Varchar>,
        payload -> Nullable<Text>,
        status -> Nullable<Int4>,
        response_headers -> Nullable<Varchar>,
        response_body -> Nullable<Text>,
        duration -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    web_hook_event_types (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    web_hook_event_types_hooks (id) {
        id -> Int4,
        web_hook_id -> Int8,
        web_hook_event_type_id -> Int8,
    }
}

table! {
    web_hooks (id) {
        id -> Int4,
        payload_url -> Varchar,
        content_type -> Int4,
        last_delivery_status -> Int4,
        status -> Int4,
        secret -> Nullable<Varchar>,
        wildcard_web_hook -> Bool,
        verify_certificate -> Bool,
        active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(spree_promotion_code_batches -> spree_promotions (promotion_id));
joinable!(spree_promotion_codes -> spree_promotion_code_batches (promotion_code_batch_id));
joinable!(spree_tax_rate_tax_categories -> spree_tax_categories (tax_category_id));
joinable!(spree_tax_rate_tax_categories -> spree_tax_rates (tax_rate_id));
joinable!(spree_wallet_payment_sources -> spree_users (user_id));

allow_tables_to_appear_in_same_query!(
    active_storage_attachments,
    active_storage_blobs,
    api_keys,
    application_requests,
    attachments,
    badge_groupings,
    badges,
    badge_types,
    cards,
    categories,
    categories_web_hooks,
    category_custom_fields,
    category_featured_topics,
    category_groups,
    category_search_data,
    category_tag_groups,
    category_tags,
    category_tag_stats,
    category_users,
    cbeta_books,
    cbeta_notes,
    cbeta_pages,
    child_themes,
    color_scheme_colors,
    color_schemes,
    custom_emojis,
    developers,
    directory_items,
    donate_payments,
    donate_projects,
    drafts,
    draft_sequences,
    email_change_requests,
    email_logs,
    email_tokens,
    embeddable_hosts,
    facebook_user_infos,
    forum_categories,
    forum_posts,
    forum_tags,
    forum_topics,
    forum_topics_tags,
    friend_links,
    friendly_id_slugs,
    github_user_infos,
    given_daily_likes,
    google_user_infos,
    group_archived_messages,
    group_custom_fields,
    group_histories,
    group_mentions,
    groups,
    groups_web_hooks,
    group_users,
    hotel_logs,
    hotel_rooms,
    incoming_domains,
    incoming_emails,
    incoming_links,
    incoming_referers,
    instagram_user_infos,
    invited_groups,
    invites,
    leave_words,
    library_books,
    library_logs,
    links,
    locales,
    mail_aliases,
    mail_domains,
    mail_users,
    mall_countries,
    mall_currencies,
    mall_states,
    mall_zone_members,
    mall_zones,
    members,
    message_bus,
    muted_users,
    notifications,
    oauth2_user_infos,
    onceoff_logs,
    optimized_images,
    permalinks,
    plugin_store_rows,
    post_actions,
    post_action_types,
    post_custom_fields,
    post_details,
    post_replies,
    post_revisions,
    posts,
    post_search_data,
    post_stats,
    post_timings,
    post_uploads,
    push_subscriptions,
    queued_posts,
    quoted_posts,
    remote_themes,
    scheduler_stats,
    schema_migration_details,
    screened_emails,
    screened_ip_addresses,
    screened_urls,
    search_logs,
    settings,
    shared_drafts,
    single_sign_on_records,
    site_settings,
    spree_addresses,
    spree_adjustment_reasons,
    spree_adjustments,
    spree_assets,
    spree_calculators,
    spree_cartons,
    spree_comments,
    spree_comment_types,
    spree_countries,
    spree_credit_cards,
    spree_customer_returns,
    spree_inventory_units,
    spree_line_item_actions,
    spree_line_items,
    spree_log_entries,
    spree_option_type_prototypes,
    spree_option_types,
    spree_option_values,
    spree_option_values_variants,
    spree_order_mutexes,
    spree_orders,
    spree_orders_promotions,
    spree_payment_capture_events,
    spree_payment_methods,
    spree_payments,
    spree_preferences,
    spree_prices,
    spree_product_option_types,
    spree_product_promotion_rules,
    spree_product_properties,
    spree_products,
    spree_products_taxons,
    spree_promotion_action_line_items,
    spree_promotion_actions,
    spree_promotion_categories,
    spree_promotion_code_batches,
    spree_promotion_codes,
    spree_promotion_rules,
    spree_promotion_rules_stores,
    spree_promotion_rules_users,
    spree_promotion_rule_taxons,
    spree_promotions,
    spree_properties,
    spree_property_prototypes,
    spree_prototypes,
    spree_prototype_taxons,
    spree_refund_reasons,
    spree_refunds,
    spree_reimbursement_credits,
    spree_reimbursements,
    spree_reimbursement_types,
    spree_relations,
    spree_relation_types,
    spree_return_authorizations,
    spree_return_items,
    spree_return_reasons,
    spree_roles,
    spree_roles_users,
    spree_shipments,
    spree_shipping_categories,
    spree_shipping_method_categories,
    spree_shipping_methods,
    spree_shipping_method_stock_locations,
    spree_shipping_method_zones,
    spree_shipping_rates,
    spree_shipping_rate_taxes,
    spree_state_changes,
    spree_states,
    spree_stock_items,
    spree_stock_locations,
    spree_stock_movements,
    spree_store_credit_categories,
    spree_store_credit_events,
    spree_store_credits,
    spree_store_credit_types,
    spree_store_credit_update_reasons,
    spree_store_payment_methods,
    spree_stores,
    spree_store_shipping_methods,
    spree_tax_categories,
    spree_taxonomies,
    spree_taxons,
    spree_tax_rates,
    spree_tax_rate_tax_categories,
    spree_unit_cancels,
    spree_user_addresses,
    spree_users,
    spree_user_stock_locations,
    spree_variant_property_rule_conditions,
    spree_variant_property_rules,
    spree_variant_property_rule_values,
    spree_variants,
    spree_wallet_payment_sources,
    spree_wished_products,
    spree_wishlists,
    spree_zone_members,
    spree_zones,
    stylesheet_cache,
    survey_fields,
    survey_forms,
    survey_records,
    tag_group_memberships,
    tag_group_permissions,
    tag_groups,
    tags,
    tag_search_data,
    tag_users,
    theme_fields,
    themes,
    theme_settings,
    todo_logs,
    todo_projects,
    todo_tasks,
    topic_allowed_groups,
    topic_allowed_users,
    topic_custom_fields,
    topic_embeds,
    topic_invites,
    topic_link_clicks,
    topic_links,
    topics,
    topic_search_data,
    topic_tags,
    topic_timers,
    topic_users,
    topic_views,
    top_topics,
    translation_overrides,
    twitter_user_infos,
    unsubscribe_keys,
    uploads,
    user_actions,
    user_api_keys,
    user_archived_messages,
    user_auth_token_logs,
    user_auth_tokens,
    user_avatars,
    user_badges,
    user_custom_fields,
    user_emails,
    user_exports,
    user_field_options,
    user_fields,
    user_histories,
    user_open_ids,
    user_options,
    user_profiles,
    user_profile_views,
    users,
    user_search_data,
    user_second_factors,
    user_stats,
    user_visits,
    user_warnings,
    votes,
    vpn_logs,
    vpn_users,
    watched_words,
    web_crawler_requests,
    web_hook_events,
    web_hook_event_types,
    web_hook_event_types_hooks,
    web_hooks,
);
