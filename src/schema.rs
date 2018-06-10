table! {
    attachments (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        size -> Int8,
        #[sql_name = "type"]
        type_ -> Varchar,
        url -> Varchar,
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
    forum_badges (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        icon -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    forum_categories (id) {
        id -> Int4,
        name -> Varchar,
        color -> Bpchar,
        description -> Nullable<Text>,
        text_color -> Bpchar,
        position -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    forum_categories_badges (id) {
        id -> Int4,
        category_id -> Int4,
        badge_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    forum_notifications (id) {
        id -> Int4,
        notification_type -> Int4,
        user_id -> Int4,
        topic_id -> Int4,
        body -> Varchar,
        read -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    forum_permalinks (id) {
        id -> Int4,
        url -> Varchar,
        resource_id -> Int4,
        resource_type -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    forum_posts (id) {
        id -> Int4,
        user_id -> Int4,
        topic_id -> Int4,
        post_id -> Nullable<Int4>,
        body -> Text,
        #[sql_name = "type"]
        type_ -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    forum_topics (id) {
        id -> Int4,
        title -> Varchar,
        user_id -> Int4,
        body -> Text,
        #[sql_name = "type"]
        type_ -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    forum_topics_badges (id) {
        id -> Int4,
        topic_id -> Int4,
        badge_id -> Int4,
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
    mall_addresses (id) {
        id -> Int4,
        user_id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        content -> Varchar,
        zip_code -> Varchar,
        phone -> Varchar,
        company -> Nullable<Varchar>,
        city -> Varchar,
        state_id -> Int4,
        country_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_adjustment_reasons (id) {
        id -> Int4,
        name -> Varchar,
        code -> Varchar,
        active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_adjustments (id) {
        id -> Int4,
        resource_type -> Varchar,
        resource_id -> Int4,
        reason -> Text,
        order_id -> Int4,
        amount -> Numeric,
        label -> Varchar,
        eligible -> Bool,
        finalized -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_assets (id) {
        id -> Int4,
        attachment_id -> Int4,
        position -> Int4,
        alt -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_calculators (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
        preferences -> Text,
        resource_type -> Varchar,
        resource_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_cartons (id) {
        id -> Int4,
        number -> Nullable<Varchar>,
        stock_id -> Int4,
        location -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_countries (id) {
        id -> Int4,
        name -> Varchar,
        iso_name -> Varchar,
        iso2 -> Varchar,
        iso3 -> Varchar,
        code -> Int4,
        states_required -> Bool,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    mall_credit_cards (id) {
        id -> Int4,
        month -> Varchar,
        year -> Varchar,
        cc_type -> Varchar,
        last_digits -> Varchar,
        gateway_customer_profile_id -> Varchar,
        gateway_payment_profile_id -> Varchar,
        name -> Varchar,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_customer_returns (id) {
        id -> Int4,
        number -> Varchar,
        stock_location -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_inventory_units (id) {
        id -> Int4,
        stock_id -> Int4,
        variant_id -> Int4,
        carton_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_line_item_actions (id) {
        id -> Int4,
        line_item_id -> Int4,
        action_id -> Int4,
        quantity -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_line_items (id) {
        id -> Int4,
        variant_id -> Int4,
        order_id -> Int4,
        quantity -> Int4,
        price -> Numeric,
        cost_price -> Numeric,
        tax_category_id -> Int4,
        adjustment_total -> Numeric,
        additional_tax_total -> Numeric,
        promo_total -> Numeric,
        included_tax_total -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_log_entries (id) {
        id -> Int4,
        resource_type -> Varchar,
        resource_id -> Int4,
        details -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_option_type_prototypes (id) {
        id -> Int4,
        prototype_id -> Int4,
        option_type_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_option_types (id) {
        id -> Int4,
        name -> Varchar,
        presentation -> Varchar,
        position -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_option_values (id) {
        id -> Int4,
        position -> Int4,
        name -> Varchar,
        presentation -> Varchar,
        option_type_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_option_values_variants (id) {
        id -> Int4,
        variant_id -> Int4,
        option_value_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_orders (id) {
        id -> Int4,
        number -> Varchar,
        item_total -> Numeric,
        total -> Numeric,
        state -> Varchar,
        adjustment_total -> Numeric,
        user_id -> Int4,
        completed_at -> Nullable<Timestamp>,
        bill_address -> Varchar,
        ship_address -> Varchar,
        payment_total -> Nullable<Numeric>,
        shipment_state -> Varchar,
        payment_state -> Varchar,
        email -> Varchar,
        special_instructions -> Nullable<Text>,
        currency -> Varchar,
        shipment_total -> Numeric,
        additional_tax_total -> Numeric,
        promo_total -> Numeric,
        included_tax_total -> Numeric,
        item_count -> Int4,
        approver_name -> Nullable<Varchar>,
        approved_at -> Nullable<Timestamp>,
        confirmation_delivered -> Bool,
        guest_token -> Varchar,
        canceled_at -> Nullable<Timestamp>,
        canceler_id -> Nullable<Int4>,
        store_id -> Int4,
        invoice_number -> Nullable<Varchar>,
        invoiced_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_orders_promotions (id) {
        id -> Int4,
        order_id -> Int4,
        promotion_id -> Int4,
        promotion_code_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_payment_capture_events (id) {
        id -> Int4,
        amount -> Numeric,
        payment_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_payment_methods (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
        name -> Varchar,
        description -> Text,
        active -> Bool,
        auto_capture -> Bool,
        preferences -> Text,
        position -> Int4,
        enable -> Bool,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_payments (id) {
        id -> Int4,
        amount -> Numeric,
        order_id -> Int4,
        payment_method_id -> Int4,
        state -> Varchar,
        response_code -> Varchar,
        avs_response -> Varchar,
        number -> Varchar,
        cvv_response_code -> Nullable<Varchar>,
        cvv_response_message -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_preferences (id) {
        id -> Int4,
        value -> Text,
        key -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_prices (id) {
        id -> Int4,
        variant_id -> Int4,
        amount -> Numeric,
        currency -> Varchar,
        country_iso -> Varchar,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_product_option_types (id) {
        id -> Int4,
        position -> Int4,
        product_id -> Int4,
        option_type_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_product_promotion_rules (id) {
        id -> Int4,
        product_id -> Nullable<Int4>,
        promotion_rule_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_product_properties (id) {
        id -> Int4,
        value -> Varchar,
        product_id -> Int4,
        property_id -> Int4,
        position -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_products (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        available_on -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        meta_description -> Nullable<Varchar>,
        meta_keywords -> Nullable<Varchar>,
        meta_title -> Nullable<Varchar>,
        tax_category_id -> Int4,
        promotionable -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_products_taxons (id) {
        id -> Int4,
        product_id -> Int4,
        taxon_id -> Int4,
        position -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_promotion_action_line_items (id) {
        id -> Int4,
        promotion_action_id -> Int4,
        variant_id -> Int4,
        quantity -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_promotion_actions (id) {
        id -> Int4,
        promotion_id -> Int4,
        position -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
        preferences -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    mall_promotion_categories (id) {
        id -> Int4,
        name -> Varchar,
        code -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_promotion_code_batches (id) {
        id -> Int4,
        promotion_id -> Int4,
        base_code -> Varchar,
        number_of_codes -> Int4,
        email -> Nullable<Varchar>,
        error -> Nullable<Varchar>,
        state -> Nullable<Varchar>,
        join_characters -> Bpchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_promotion_codes (id) {
        id -> Int4,
        promotion_id -> Int4,
        value -> Varchar,
        promotion_code_batch_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_promotion_rules (id) {
        id -> Int4,
        promotion_id -> Int4,
        product_group_id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
        code -> Varchar,
        preferences -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_promotion_rules_stores (id) {
        id -> Int4,
        store_id -> Int4,
        promotion_rule_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_promotion_rule_taxons (id) {
        id -> Int4,
        taxon_id -> Nullable<Int4>,
        promotion_rule_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_promotions (id) {
        id -> Int4,
        description -> Varchar,
        expires_at -> Nullable<Timestamp>,
        starts_at -> Nullable<Timestamp>,
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        usage_limit -> Int4,
        match_policy -> Varchar,
        code -> Varchar,
        advertise -> Bool,
        path -> Varchar,
        promotion_category_id -> Int4,
        per_code_usage_limit -> Int4,
        apply_automatically -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_properties (id) {
        id -> Int4,
        name -> Varchar,
        presentation -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_property_prototypes (id) {
        id -> Int4,
        prototype_id -> Int4,
        property_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_prototypes (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_prototype_taxons (id) {
        id -> Int4,
        taxon_id -> Int4,
        prototype_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_refund_reasons (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        active -> Bool,
        mutable -> Bool,
        code -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_refunds (id) {
        id -> Int4,
        payment_id -> Int4,
        amount -> Numeric,
        transaction_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        refund_reason_id -> Int4,
        reimbursement_id -> Int4,
    }
}

table! {
    mall_reimbursement_credits (id) {
        id -> Int4,
        amount -> Numeric,
        reimbursement_id -> Int4,
        creditable_id -> Int4,
        creditable_type -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_reimbursements (id) {
        id -> Int4,
        number -> Varchar,
        reimbursement_status -> Varchar,
        customer_return_id -> Int4,
        order_id -> Int4,
        total -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_reimbursement_types (id) {
        id -> Int4,
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        active -> Bool,
        mutable -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_relations (id) {
        id -> Int4,
        relation_type_id -> Int4,
        relatable_type -> Varchar,
        relatable_id -> Int4,
        related_to_type -> Varchar,
        related_to_id -> Int4,
        position -> Int4,
        discount_amount -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_relation_types (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        applies_to -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_return_authorizations (id) {
        id -> Int4,
        number -> Varchar,
        state -> Varchar,
        order_id -> Int4,
        memo -> Nullable<Text>,
        stock_location_id -> Int4,
        return_reason_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_return_items (id) {
        id -> Int4,
        return_authorization_id -> Int4,
        inventory_unit_id -> Int4,
        exchange_variant_id -> Int4,
        amount -> Numeric,
        included_tax_total -> Numeric,
        additional_tax_total -> Numeric,
        reception_status -> Varchar,
        acceptance_status -> Varchar,
        customer_return_id -> Int4,
        reimbursement_id -> Int4,
        exchange_inventory_unit_id -> Int4,
        acceptance_status_errors -> Nullable<Text>,
        preferred_reimbursement_type_id -> Int4,
        override_reimbursement_type_id -> Int4,
        resellable -> Bool,
        return_reason_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_return_reasons (id) {
        id -> Int4,
        name -> Varchar,
        active -> Bool,
        mutable -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_shipments (id) {
        id -> Int4,
        tracking -> Nullable<Varchar>,
        number -> Nullable<Varchar>,
        cost -> Nullable<Numeric>,
        shipped_at -> Timestamp,
        order_id -> Nullable<Int4>,
        deprecated_address_id -> Nullable<Int4>,
        state -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        stock_location_id -> Nullable<Int4>,
        adjustment_total -> Nullable<Numeric>,
        additional_tax_total -> Nullable<Numeric>,
        promo_total -> Nullable<Numeric>,
        included_tax_total -> Numeric,
    }
}

table! {
    mall_shipping_methods (id) {
        id -> Int4,
        name -> Varchar,
        tracking_url -> Nullable<Varchar>,
        tax_category_id -> Int4,
        code -> Varchar,
        available_to_all -> Bool,
        carrier -> Varchar,
        service_level -> Varchar,
        available_to_users -> Bool,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_shipping_method_stock_locations (id) {
        id -> Int4,
        shipping_method_id -> Int4,
        stock_location_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_shipping_method_zones (id) {
        id -> Int4,
        shipping_method_id -> Int4,
        zone_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_shipping_rates (id) {
        id -> Int4,
        shipment_id -> Int4,
        shipping_method_id -> Int4,
        selected -> Bool,
        cost -> Numeric,
        tax_rate_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_shipping_rate_taxes (id) {
        id -> Int4,
        amount -> Numeric,
        tax_rate_id -> Int4,
        shipping_rate_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_state_changes (id) {
        id -> Int4,
        name -> Varchar,
        previous_state -> Varchar,
        stateful_id -> Int4,
        user_id -> Int4,
        stateful_type -> Varchar,
        next_state -> Varchar,
        created_at -> Timestamp,
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
        created_at -> Timestamp,
    }
}

table! {
    mall_stock_items (id) {
        id -> Int4,
        stock_location_id -> Int4,
        variant_id -> Int4,
        count_on_hand -> Int4,
        backorderable -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Timestamp,
    }
}

table! {
    mall_stock_locations (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        default -> Bool,
        address_id -> Int4,
        active -> Bool,
        backorderable_default -> Bool,
        propagate_all_variants -> Bool,
        position -> Int4,
        restock_inventory -> Bool,
        fulfillable -> Bool,
        code -> Varchar,
        check_stock_on_transfer -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_stock_movements (id) {
        id -> Int4,
        stock_item_id -> Int4,
        quantity -> Int4,
        action -> Varchar,
        originator_type -> Varchar,
        originator_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_store_credit_events (id) {
        id -> Int4,
        store_credit_id -> Int4,
        action -> Varchar,
        amount -> Numeric,
        user_total_amount -> Numeric,
        authorization_code -> Varchar,
        deleted_at -> Timestamp,
        originator_type -> Varchar,
        originator_id -> Int4,
        update_reason_id -> Int4,
        amount_remaining -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_store_credits (id) {
        id -> Int4,
        user_id -> Int4,
        amount -> Numeric,
        amount_used -> Numeric,
        amount_authorized -> Numeric,
        currency -> Varchar,
        memo -> Nullable<Text>,
        type_id -> Int4,
        deleted_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        invalidated_at -> Date,
    }
}

table! {
    mall_store_payment_methods (id) {
        id -> Int4,
        store_id -> Int4,
        payment_method_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_stores (id) {
        id -> Int4,
        name -> Varchar,
        url -> Varchar,
        meta_description -> Nullable<Varchar>,
        meta_keywords -> Nullable<Varchar>,
        meta_title -> Nullable<Varchar>,
        mail_from_address_id -> Int4,
        default_currency -> Varchar,
        code -> Varchar,
        default -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_store_shipping_methods (id) {
        id -> Int4,
        store_id -> Int4,
        shipping_method_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_tax_rates (id) {
        id -> Int4,
        amount -> Numeric,
        zone_id -> Int4,
        name -> Varchar,
        show_rate_in_label -> Nullable<Bool>,
        deleted_at -> Nullable<Timestamp>,
        starts_at -> Timestamp,
        expires_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_variants (id) {
        id -> Int4,
        sku -> Varchar,
        weight -> Numeric,
        height -> Numeric,
        width -> Numeric,
        depth -> Numeric,
        is_master -> Bool,
        product_id -> Int4,
        cost_price -> Numeric,
        position -> Int4,
        cost_currency -> Varchar,
        track_inventory -> Bool,
        tax_category_id -> Int4,
        deleted_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    mall_wishlists (id) {
        id -> Int4,
        variant_id -> Int4,
        user_id -> Int4,
        remark -> Nullable<Text>,
        quantity -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_zone_members (id) {
        id -> Int4,
        zoneable_type -> Varchar,
        zoneable_id -> Int4,
        zone_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    mall_zones (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        zone_members_count -> Int4,
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
        received -> Numeric,
        send -> Numeric,
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
    forum_badges,
    forum_categories,
    forum_categories_badges,
    forum_notifications,
    forum_permalinks,
    forum_posts,
    forum_topics,
    forum_topics_badges,
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
    mall_addresses,
    mall_adjustment_reasons,
    mall_adjustments,
    mall_assets,
    mall_calculators,
    mall_cartons,
    mall_countries,
    mall_credit_cards,
    mall_customer_returns,
    mall_inventory_units,
    mall_line_item_actions,
    mall_line_items,
    mall_log_entries,
    mall_option_type_prototypes,
    mall_option_types,
    mall_option_values,
    mall_option_values_variants,
    mall_orders,
    mall_orders_promotions,
    mall_payment_capture_events,
    mall_payment_methods,
    mall_payments,
    mall_preferences,
    mall_prices,
    mall_product_option_types,
    mall_product_promotion_rules,
    mall_product_properties,
    mall_products,
    mall_products_taxons,
    mall_promotion_action_line_items,
    mall_promotion_actions,
    mall_promotion_categories,
    mall_promotion_code_batches,
    mall_promotion_codes,
    mall_promotion_rules,
    mall_promotion_rules_stores,
    mall_promotion_rule_taxons,
    mall_promotions,
    mall_properties,
    mall_property_prototypes,
    mall_prototypes,
    mall_prototype_taxons,
    mall_refund_reasons,
    mall_refunds,
    mall_reimbursement_credits,
    mall_reimbursements,
    mall_reimbursement_types,
    mall_relations,
    mall_relation_types,
    mall_return_authorizations,
    mall_return_items,
    mall_return_reasons,
    mall_shipments,
    mall_shipping_methods,
    mall_shipping_method_stock_locations,
    mall_shipping_method_zones,
    mall_shipping_rates,
    mall_shipping_rate_taxes,
    mall_state_changes,
    mall_states,
    mall_stock_items,
    mall_stock_locations,
    mall_stock_movements,
    mall_store_credit_events,
    mall_store_credits,
    mall_store_payment_methods,
    mall_stores,
    mall_store_shipping_methods,
    mall_tax_rates,
    mall_variants,
    mall_wishlists,
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
