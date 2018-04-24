table! {
    currencies (id) {
        id -> Int8,
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
    forum_categories (id) {
        id -> Int8,
        name -> Varchar,
        color -> Varchar,
        icon -> Varchar,
        description -> Nullable<Varchar>,
        parent_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    forum_posts (id) {
        id -> Int8,
        body -> Text,
        user_id -> Int8,
        post_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    forum_tags (id) {
        id -> Int8,
        name -> Varchar,
        background_color -> Varchar,
        text_color -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    forum_topics (id) {
        id -> Int8,
        title -> Varchar,
        body -> Text,
        user_id -> Int8,
        category_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    forum_topics_tags (id) {
        id -> Int8,
        topic_id -> Int8,
        tag_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    friendly_id_slugs (id) {
        id -> Int8,
        slug -> Varchar,
        sluggable_id -> Int8,
        sluggable_type -> Varchar,
        scope -> Varchar,
        created_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
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
        created_at -> Timestamptz,
    }
}

table! {
    settings (id) {
        id -> Int8,
        key -> Varchar,
        salt -> Nullable<Bytea>,
        value -> Bytea,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_addresses (id) {
        id -> Int8,
        firstname -> Varchar,
        lastname -> Varchar,
        address1 -> Varchar,
        address2 -> Nullable<Varchar>,
        city -> Varchar,
        zipcode -> Varchar,
        phone -> Varchar,
        state_name -> Varchar,
        alternative_phone -> Nullable<Varchar>,
        company -> Nullable<Varchar>,
        state_id -> Int8,
        country_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_adjustments (id) {
        id -> Int8,
        source_type -> Varchar,
        source_id -> Int8,
        adjustable_type -> Varchar,
        adjustable_id -> Int8,
        amount -> Numeric,
        label -> Varchar,
        mandatory -> Nullable<Bool>,
        eligible -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        state -> Nullable<Varchar>,
        order_id -> Int8,
        included -> Nullable<Bool>,
    }
}

table! {
    spree_assets (id) {
        id -> Int8,
        viewable_type -> Nullable<Varchar>,
        viewable_id -> Nullable<Int8>,
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
        id -> Int8,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        calculable_type -> Nullable<Varchar>,
        calculable_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        preferences -> Nullable<Text>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_countries (id) {
        id -> Int8,
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
    spree_credit_cards (id) {
        id -> Int8,
        month -> Nullable<Varchar>,
        year -> Nullable<Varchar>,
        cc_type -> Nullable<Varchar>,
        last_digits -> Nullable<Varchar>,
        address_id -> Nullable<Int8>,
        gateway_customer_profile_id -> Nullable<Varchar>,
        gateway_payment_profile_id -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name -> Nullable<Varchar>,
        user_id -> Nullable<Int8>,
        payment_method_id -> Nullable<Int8>,
        default -> Bool,
    }
}

table! {
    spree_customer_returns (id) {
        id -> Int8,
        number -> Nullable<Varchar>,
        stock_location_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_gateways (id) {
        id -> Int8,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        active -> Nullable<Bool>,
        environment -> Nullable<Varchar>,
        server -> Nullable<Varchar>,
        test_mode -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        preferences -> Nullable<Text>,
    }
}

table! {
    spree_inventory_units (id) {
        id -> Int8,
        state -> Nullable<Varchar>,
        variant_id -> Nullable<Int8>,
        order_id -> Nullable<Int8>,
        shipment_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        pending -> Nullable<Bool>,
        line_item_id -> Nullable<Int8>,
        quantity -> Nullable<Int4>,
        original_return_item_id -> Nullable<Int8>,
    }
}

table! {
    spree_line_items (id) {
        id -> Int8,
        variant_id -> Nullable<Int8>,
        order_id -> Nullable<Int8>,
        quantity -> Int4,
        price -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        currency -> Nullable<Varchar>,
        cost_price -> Nullable<Numeric>,
        tax_category_id -> Nullable<Int8>,
        adjustment_total -> Nullable<Numeric>,
        additional_tax_total -> Nullable<Numeric>,
        promo_total -> Nullable<Numeric>,
        included_tax_total -> Numeric,
        pre_tax_amount -> Numeric,
        taxable_adjustment_total -> Numeric,
        non_taxable_adjustment_total -> Numeric,
    }
}

table! {
    spree_log_entries (id) {
        id -> Int8,
        source_type -> Nullable<Varchar>,
        source_id -> Nullable<Int8>,
        details -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_option_type_prototypes (id) {
        id -> Int8,
        prototype_id -> Nullable<Int8>,
        option_type_id -> Nullable<Int8>,
    }
}

table! {
    spree_option_types (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        presentation -> Nullable<Varchar>,
        position -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_option_values (id) {
        id -> Int8,
        position -> Nullable<Int4>,
        name -> Nullable<Varchar>,
        presentation -> Nullable<Varchar>,
        option_type_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_option_value_variants (id) {
        id -> Int8,
        variant_id -> Nullable<Int8>,
        option_value_id -> Nullable<Int8>,
    }
}

table! {
    spree_order_promotions (id) {
        id -> Int8,
        order_id -> Nullable<Int8>,
        promotion_id -> Nullable<Int8>,
    }
}

table! {
    spree_orders (id) {
        id -> Int8,
        number -> Nullable<Varchar>,
        item_total -> Numeric,
        total -> Numeric,
        state -> Nullable<Varchar>,
        adjustment_total -> Numeric,
        user_id -> Nullable<Int8>,
        completed_at -> Nullable<Timestamp>,
        bill_address_id -> Nullable<Int8>,
        ship_address_id -> Nullable<Int8>,
        payment_total -> Nullable<Numeric>,
        shipment_state -> Nullable<Varchar>,
        payment_state -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        special_instructions -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        currency -> Nullable<Varchar>,
        last_ip_address -> Nullable<Varchar>,
        created_by_id -> Nullable<Int8>,
        shipment_total -> Numeric,
        additional_tax_total -> Nullable<Numeric>,
        promo_total -> Nullable<Numeric>,
        channel -> Nullable<Varchar>,
        included_tax_total -> Numeric,
        item_count -> Nullable<Int4>,
        approver_id -> Nullable<Int8>,
        approved_at -> Nullable<Timestamp>,
        confirmation_delivered -> Nullable<Bool>,
        considered_risky -> Nullable<Bool>,
        guest_token -> Nullable<Varchar>,
        canceled_at -> Nullable<Timestamp>,
        canceler_id -> Nullable<Int8>,
        store_id -> Nullable<Int8>,
        state_lock_version -> Int4,
        taxable_adjustment_total -> Numeric,
        non_taxable_adjustment_total -> Numeric,
    }
}

table! {
    spree_payment_capture_events (id) {
        id -> Int8,
        amount -> Nullable<Numeric>,
        payment_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_payment_methods (id) {
        id -> Int8,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        active -> Nullable<Bool>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        display_on -> Nullable<Varchar>,
        auto_capture -> Nullable<Bool>,
        preferences -> Nullable<Text>,
        position -> Nullable<Int4>,
    }
}

table! {
    spree_payments (id) {
        id -> Int8,
        amount -> Numeric,
        order_id -> Nullable<Int8>,
        source_type -> Nullable<Varchar>,
        source_id -> Nullable<Int8>,
        payment_method_id -> Nullable<Int8>,
        state -> Nullable<Varchar>,
        response_code -> Nullable<Varchar>,
        avs_response -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        number -> Nullable<Varchar>,
        cvv_response_code -> Nullable<Varchar>,
        cvv_response_message -> Nullable<Varchar>,
    }
}

table! {
    spree_preferences (id) {
        id -> Int8,
        value -> Nullable<Text>,
        key -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_prices (id) {
        id -> Int8,
        variant_id -> Int8,
        amount -> Nullable<Numeric>,
        currency -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_product_option_types (id) {
        id -> Int8,
        position -> Nullable<Int4>,
        product_id -> Nullable<Int8>,
        option_type_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_product_promotion_rules (id) {
        id -> Int8,
        product_id -> Nullable<Int8>,
        promotion_rule_id -> Nullable<Int8>,
    }
}

table! {
    spree_product_properties (id) {
        id -> Int8,
        value -> Nullable<Varchar>,
        product_id -> Nullable<Int8>,
        property_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        position -> Nullable<Int4>,
    }
}

table! {
    spree_products (id) {
        id -> Int8,
        name -> Varchar,
        description -> Nullable<Text>,
        available_on -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        slug -> Nullable<Varchar>,
        meta_description -> Nullable<Text>,
        meta_keywords -> Nullable<Varchar>,
        tax_category_id -> Nullable<Int8>,
        shipping_category_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        promotionable -> Nullable<Bool>,
        meta_title -> Nullable<Varchar>,
        discontinue_on -> Nullable<Timestamp>,
    }
}

table! {
    spree_products_taxons (id) {
        id -> Int8,
        product_id -> Nullable<Int8>,
        taxon_id -> Nullable<Int8>,
        position -> Nullable<Int4>,
    }
}

table! {
    spree_promotion_action_line_items (id) {
        id -> Int8,
        promotion_action_id -> Nullable<Int8>,
        variant_id -> Nullable<Int8>,
        quantity -> Nullable<Int4>,
    }
}

table! {
    spree_promotion_actions (id) {
        id -> Int8,
        promotion_id -> Nullable<Int8>,
        position -> Nullable<Int4>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_promotion_categories (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        code -> Nullable<Varchar>,
    }
}

table! {
    spree_promotion_rules (id) {
        id -> Int8,
        promotion_id -> Nullable<Int8>,
        user_id -> Nullable<Int8>,
        product_group_id -> Nullable<Int8>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        code -> Nullable<Varchar>,
        preferences -> Nullable<Text>,
    }
}

table! {
    spree_promotion_rule_taxons (id) {
        id -> Int8,
        taxon_id -> Nullable<Int8>,
        promotion_rule_id -> Nullable<Int8>,
    }
}

table! {
    spree_promotion_rule_users (id) {
        id -> Int8,
        user_id -> Nullable<Int8>,
        promotion_rule_id -> Nullable<Int8>,
    }
}

table! {
    spree_promotions (id) {
        id -> Int8,
        description -> Text,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
        promotion_category_id -> Nullable<Int8>,
    }
}

table! {
    spree_properties (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        presentation -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_property_prototypes (id) {
        id -> Int8,
        prototype_id -> Nullable<Int8>,
        property_id -> Nullable<Int8>,
    }
}

table! {
    spree_prototypes (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_prototype_taxons (id) {
        id -> Int8,
        taxon_id -> Nullable<Int8>,
        prototype_id -> Nullable<Int8>,
    }
}

table! {
    spree_refund_reasons (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        mutable -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_refunds (id) {
        id -> Int8,
        payment_id -> Nullable<Int8>,
        amount -> Numeric,
        transaction_id -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        refund_reason_id -> Nullable<Int8>,
        reimbursement_id -> Nullable<Int8>,
    }
}

table! {
    spree_reimbursement_credits (id) {
        id -> Int8,
        amount -> Numeric,
        reimbursement_id -> Nullable<Int8>,
        creditable_id -> Nullable<Int8>,
        creditable_type -> Nullable<Varchar>,
    }
}

table! {
    spree_reimbursements (id) {
        id -> Int8,
        number -> Nullable<Varchar>,
        reimbursement_status -> Nullable<Varchar>,
        customer_return_id -> Nullable<Int8>,
        order_id -> Nullable<Int8>,
        total -> Nullable<Numeric>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_reimbursement_types (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        mutable -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
    }
}

table! {
    spree_return_authorization_reasons (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        mutable -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_return_authorizations (id) {
        id -> Int8,
        number -> Nullable<Varchar>,
        state -> Nullable<Varchar>,
        order_id -> Nullable<Int8>,
        memo -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        stock_location_id -> Nullable<Int8>,
        return_authorization_reason_id -> Nullable<Int8>,
    }
}

table! {
    spree_return_items (id) {
        id -> Int8,
        return_authorization_id -> Nullable<Int8>,
        inventory_unit_id -> Nullable<Int8>,
        exchange_variant_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        pre_tax_amount -> Numeric,
        included_tax_total -> Numeric,
        additional_tax_total -> Numeric,
        reception_status -> Nullable<Varchar>,
        acceptance_status -> Nullable<Varchar>,
        customer_return_id -> Nullable<Int8>,
        reimbursement_id -> Nullable<Int8>,
        acceptance_status_errors -> Nullable<Text>,
        preferred_reimbursement_type_id -> Nullable<Int8>,
        override_reimbursement_type_id -> Nullable<Int8>,
        resellable -> Bool,
    }
}

table! {
    spree_roles (id) {
        id -> Int8,
        name -> Varchar,
    }
}

table! {
    spree_role_users (id) {
        id -> Int8,
        role_id -> Int8,
        user_id -> Int8,
        nbf -> Date,
        exp -> Date,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_shipments (id) {
        id -> Int8,
        tracking -> Nullable<Varchar>,
        number -> Nullable<Varchar>,
        cost -> Nullable<Numeric>,
        shipped_at -> Nullable<Timestamp>,
        order_id -> Nullable<Int8>,
        address_id -> Nullable<Int8>,
        state -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        stock_location_id -> Nullable<Int8>,
        adjustment_total -> Nullable<Numeric>,
        additional_tax_total -> Nullable<Numeric>,
        promo_total -> Nullable<Numeric>,
        included_tax_total -> Numeric,
        pre_tax_amount -> Numeric,
        taxable_adjustment_total -> Numeric,
        non_taxable_adjustment_total -> Numeric,
    }
}

table! {
    spree_shipping_categories (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_shipping_method_categories (id) {
        id -> Int8,
        shipping_method_id -> Int8,
        shipping_category_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_shipping_methods (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        display_on -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        tracking_url -> Nullable<Varchar>,
        admin_name -> Nullable<Varchar>,
        tax_category_id -> Nullable<Int8>,
        code -> Nullable<Varchar>,
    }
}

table! {
    spree_shipping_method_zones (id) {
        id -> Int8,
        shipping_method_id -> Nullable<Int8>,
        zone_id -> Nullable<Int8>,
    }
}

table! {
    spree_shipping_rates (id) {
        id -> Int8,
        shipment_id -> Nullable<Int8>,
        shipping_method_id -> Nullable<Int8>,
        selected -> Nullable<Bool>,
        cost -> Nullable<Numeric>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        tax_rate_id -> Nullable<Int8>,
    }
}

table! {
    spree_state_changes (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        previous_state -> Nullable<Varchar>,
        stateful_id -> Nullable<Int8>,
        user_id -> Nullable<Int8>,
        stateful_type -> Nullable<Varchar>,
        next_state -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_states (id) {
        id -> Int8,
        name -> Varchar,
        abbr -> Varchar,
        country_id -> Int8,
        updated_at -> Timestamp,
    }
}

table! {
    spree_stock_items (id) {
        id -> Int8,
        stock_location_id -> Nullable<Int8>,
        variant_id -> Nullable<Int8>,
        count_on_hand -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        backorderable -> Nullable<Bool>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_stock_locations (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        default -> Bool,
        address1 -> Nullable<Varchar>,
        address2 -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        state_id -> Nullable<Int8>,
        state_name -> Nullable<Varchar>,
        country_id -> Nullable<Int8>,
        zipcode -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        backorderable_default -> Nullable<Bool>,
        propagate_all_variants -> Nullable<Bool>,
        admin_name -> Nullable<Varchar>,
    }
}

table! {
    spree_stock_movements (id) {
        id -> Int8,
        stock_item_id -> Nullable<Int8>,
        quantity -> Nullable<Int4>,
        action -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        originator_type -> Nullable<Varchar>,
        originator_id -> Nullable<Int8>,
    }
}

table! {
    spree_stock_transfers (id) {
        id -> Int8,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        reference -> Nullable<Varchar>,
        source_location_id -> Nullable<Int8>,
        destination_location_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        number -> Nullable<Varchar>,
    }
}

table! {
    spree_store_credit_categories (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_store_credit_events (id) {
        id -> Int8,
        store_credit_id -> Int8,
        action -> Varchar,
        amount -> Nullable<Numeric>,
        authorization_code -> Varchar,
        user_total_amount -> Numeric,
        originator_id -> Nullable<Int8>,
        originator_type -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_store_credits (id) {
        id -> Int8,
        user_id -> Nullable<Int8>,
        category_id -> Nullable<Int8>,
        created_by_id -> Nullable<Int8>,
        amount -> Numeric,
        amount_used -> Numeric,
        memo -> Nullable<Text>,
        deleted_at -> Nullable<Timestamp>,
        currency -> Nullable<Varchar>,
        amount_authorized -> Numeric,
        originator_id -> Nullable<Int8>,
        originator_type -> Nullable<Varchar>,
        type_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_store_credit_types (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        priority -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_stores (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        url -> Nullable<Varchar>,
        meta_description -> Nullable<Text>,
        meta_keywords -> Nullable<Text>,
        seo_title -> Nullable<Varchar>,
        mail_from_address -> Nullable<Varchar>,
        default_currency -> Nullable<Varchar>,
        code -> Nullable<Varchar>,
        default -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_taggings (id) {
        id -> Int8,
        tag_id -> Nullable<Int8>,
        taggable_type -> Nullable<Varchar>,
        taggable_id -> Nullable<Int8>,
        tagger_type -> Nullable<Varchar>,
        tagger_id -> Nullable<Int8>,
        context -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_tags (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        taggings_count -> Nullable<Int4>,
    }
}

table! {
    spree_tax_categories (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        description -> Text,
        is_default -> Nullable<Bool>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        tax_code -> Nullable<Varchar>,
    }
}

table! {
    spree_taxonomies (id) {
        id -> Int8,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        position -> Nullable<Int4>,
    }
}

table! {
    spree_taxons (id) {
        id -> Int8,
        parent_id -> Nullable<Int8>,
        position -> Nullable<Int4>,
        name -> Varchar,
        permalink -> Nullable<Varchar>,
        taxonomy_id -> Nullable<Int8>,
        lft -> Nullable<Int4>,
        rgt -> Nullable<Int4>,
        icon_file_name -> Nullable<Varchar>,
        icon_content_type -> Nullable<Varchar>,
        icon_file_size -> Nullable<Int4>,
        icon_updated_at -> Nullable<Timestamp>,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        meta_title -> Nullable<Varchar>,
        meta_description -> Text,
        meta_keywords -> Nullable<Varchar>,
        depth -> Nullable<Int4>,
    }
}

table! {
    spree_tax_rates (id) {
        id -> Int8,
        amount -> Nullable<Numeric>,
        zone_id -> Nullable<Int8>,
        tax_category_id -> Nullable<Int8>,
        included_in_price -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name -> Nullable<Varchar>,
        show_rate_in_label -> Nullable<Bool>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_trackers (id) {
        id -> Int8,
        analytics_id -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        engine -> Int4,
    }
}

table! {
    spree_users (id) {
        id -> Int8,
        encrypted_password -> Varchar,
        password_salt -> Nullable<Varchar>,
        email -> Varchar,
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
        ship_address_id -> Nullable<Int8>,
        bill_address_id -> Nullable<Int8>,
        authentication_token -> Nullable<Varchar>,
        unlock_token -> Nullable<Varchar>,
        locked_at -> Nullable<Timestamp>,
        reset_password_sent_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        spree_api_key -> Nullable<Varchar>,
        remember_created_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        confirmation_token -> Nullable<Varchar>,
        confirmed_at -> Nullable<Timestamp>,
        confirmation_sent_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_variants (id) {
        id -> Int8,
        sku -> Varchar,
        weight -> Nullable<Numeric>,
        height -> Nullable<Numeric>,
        width -> Nullable<Numeric>,
        depth -> Nullable<Numeric>,
        deleted_at -> Nullable<Timestamp>,
        is_master -> Nullable<Bool>,
        product_id -> Nullable<Int8>,
        cost_price -> Nullable<Numeric>,
        position -> Nullable<Int4>,
        cost_currency -> Nullable<Varchar>,
        track_inventory -> Nullable<Bool>,
        tax_category_id -> Nullable<Int8>,
        updated_at -> Timestamp,
        discontinue_on -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

table! {
    spree_zone_members (id) {
        id -> Int8,
        zone_id -> Int8,
        zoneable_type -> Varchar,
        zoneable_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_zones (id) {
        id -> Int8,
        name -> Varchar,
        description -> Text,
        kind -> Varchar,
        default_tax -> Bool,
        zone_members_count -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    votes (id) {
        id -> Int8,
        resource_type -> Varchar,
        resource_id -> Int8,
        point -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    currencies,
    forum_categories,
    forum_posts,
    forum_tags,
    forum_topics,
    forum_topics_tags,
    friendly_id_slugs,
    locales,
    logs,
    settings,
    spree_addresses,
    spree_adjustments,
    spree_assets,
    spree_calculators,
    spree_countries,
    spree_credit_cards,
    spree_customer_returns,
    spree_gateways,
    spree_inventory_units,
    spree_line_items,
    spree_log_entries,
    spree_option_type_prototypes,
    spree_option_types,
    spree_option_values,
    spree_option_value_variants,
    spree_order_promotions,
    spree_orders,
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
    spree_promotion_rules,
    spree_promotion_rule_taxons,
    spree_promotion_rule_users,
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
    spree_return_authorization_reasons,
    spree_return_authorizations,
    spree_return_items,
    spree_roles,
    spree_role_users,
    spree_shipments,
    spree_shipping_categories,
    spree_shipping_method_categories,
    spree_shipping_methods,
    spree_shipping_method_zones,
    spree_shipping_rates,
    spree_state_changes,
    spree_states,
    spree_stock_items,
    spree_stock_locations,
    spree_stock_movements,
    spree_stock_transfers,
    spree_store_credit_categories,
    spree_store_credit_events,
    spree_store_credits,
    spree_store_credit_types,
    spree_stores,
    spree_taggings,
    spree_tags,
    spree_tax_categories,
    spree_taxonomies,
    spree_taxons,
    spree_tax_rates,
    spree_trackers,
    spree_users,
    spree_variants,
    spree_zone_members,
    spree_zones,
    votes,
);
