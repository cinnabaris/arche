table! {
    friendly_id_slugs (id) {
        id -> Int4,
        slug -> Varchar,
        sluggable_id -> Int4,
        sluggable_type -> Nullable<Varchar>,
        scope -> Nullable<Varchar>,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_adjustments (id) {
        id -> Int4,
        source_type -> Nullable<Varchar>,
        source_id -> Nullable<Int4>,
        adjustable_type -> Nullable<Varchar>,
        adjustable_id -> Nullable<Int4>,
        amount -> Nullable<Numeric>,
        label -> Nullable<Varchar>,
        mandatory -> Nullable<Bool>,
        eligible -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        state -> Nullable<Varchar>,
        order_id -> Int4,
        included -> Nullable<Bool>,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
        preferences -> Nullable<Text>,
        deleted_at -> Nullable<Timestamp>,
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
        zipcode_required -> Nullable<Bool>,
    }
}

table! {
    spree_credit_cards (id) {
        id -> Int4,
        month -> Nullable<Varchar>,
        year -> Nullable<Varchar>,
        cc_type -> Nullable<Varchar>,
        last_digits -> Nullable<Varchar>,
        address_id -> Nullable<Int4>,
        gateway_customer_profile_id -> Nullable<Varchar>,
        gateway_payment_profile_id -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        name -> Nullable<Varchar>,
        user_id -> Nullable<Int4>,
        payment_method_id -> Nullable<Int4>,
        default -> Bool,
    }
}

table! {
    spree_customer_returns (id) {
        id -> Int4,
        number -> Nullable<Varchar>,
        stock_location_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_gateways (id) {
        id -> Int4,
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
        id -> Int4,
        state -> Nullable<Varchar>,
        variant_id -> Nullable<Int4>,
        order_id -> Nullable<Int4>,
        shipment_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        pending -> Nullable<Bool>,
        line_item_id -> Nullable<Int4>,
        quantity -> Nullable<Int4>,
        original_return_item_id -> Nullable<Int4>,
    }
}

table! {
    spree_line_items (id) {
        id -> Int4,
        variant_id -> Nullable<Int4>,
        order_id -> Nullable<Int4>,
        quantity -> Int4,
        price -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        currency -> Nullable<Varchar>,
        cost_price -> Nullable<Numeric>,
        tax_category_id -> Nullable<Int4>,
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
        id -> Int4,
        source_type -> Nullable<Varchar>,
        source_id -> Nullable<Int4>,
        details -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_option_type_prototypes (id) {
        prototype_id -> Nullable<Int4>,
        option_type_id -> Nullable<Int4>,
        id -> Int8,
    }
}

table! {
    spree_option_types (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        presentation -> Nullable<Varchar>,
        position -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_option_values (id) {
        id -> Int4,
        position -> Nullable<Int4>,
        name -> Nullable<Varchar>,
        presentation -> Nullable<Varchar>,
        option_type_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_option_value_variants (id) {
        variant_id -> Nullable<Int4>,
        option_value_id -> Nullable<Int4>,
        id -> Int8,
    }
}

table! {
    spree_order_promotions (id) {
        order_id -> Nullable<Int4>,
        promotion_id -> Nullable<Int4>,
        id -> Int8,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
        considered_risky -> Nullable<Bool>,
        guest_token -> Nullable<Varchar>,
        canceled_at -> Nullable<Timestamp>,
        canceler_id -> Nullable<Int4>,
        store_id -> Nullable<Int4>,
        state_lock_version -> Int4,
        taxable_adjustment_total -> Numeric,
        non_taxable_adjustment_total -> Numeric,
    }
}

table! {
    spree_payment_capture_events (id) {
        id -> Int4,
        amount -> Nullable<Numeric>,
        payment_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
        id -> Int4,
        amount -> Numeric,
        order_id -> Nullable<Int4>,
        source_type -> Nullable<Varchar>,
        source_id -> Nullable<Int4>,
        payment_method_id -> Nullable<Int4>,
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
        id -> Int4,
        value -> Nullable<Text>,
        key -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_prices (id) {
        id -> Int4,
        variant_id -> Int4,
        amount -> Nullable<Numeric>,
        currency -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_product_option_types (id) {
        id -> Int4,
        position -> Nullable<Int4>,
        product_id -> Nullable<Int4>,
        option_type_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_product_promotion_rules (id) {
        product_id -> Nullable<Int4>,
        promotion_rule_id -> Nullable<Int4>,
        id -> Int8,
    }
}

table! {
    spree_product_properties (id) {
        id -> Int4,
        value -> Nullable<Varchar>,
        product_id -> Nullable<Int4>,
        property_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
        promotionable -> Nullable<Bool>,
        meta_title -> Nullable<Varchar>,
        discontinue_on -> Nullable<Timestamp>,
    }
}

table! {
    spree_products_taxons (id) {
        product_id -> Nullable<Int4>,
        taxon_id -> Nullable<Int4>,
        id -> Int8,
        position -> Nullable<Int4>,
    }
}

table! {
    spree_promotion_action_line_items (id) {
        id -> Int4,
        promotion_action_id -> Nullable<Int4>,
        variant_id -> Nullable<Int4>,
        quantity -> Nullable<Int4>,
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
    }
}

table! {
    spree_promotion_categories (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        code -> Nullable<Varchar>,
    }
}

table! {
    spree_promotion_rules (id) {
        id -> Int4,
        promotion_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        product_group_id -> Nullable<Int4>,
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
        id -> Int4,
        taxon_id -> Nullable<Int4>,
        promotion_rule_id -> Nullable<Int4>,
    }
}

table! {
    spree_promotion_rule_users (id) {
        user_id -> Nullable<Int4>,
        promotion_rule_id -> Nullable<Int4>,
        id -> Int8,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
        promotion_category_id -> Nullable<Int4>,
    }
}

table! {
    spree_properties (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        presentation -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_property_prototypes (id) {
        prototype_id -> Nullable<Int4>,
        property_id -> Nullable<Int4>,
        id -> Int8,
    }
}

table! {
    spree_prototypes (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_prototype_taxons (id) {
        id -> Int4,
        taxon_id -> Nullable<Int4>,
        prototype_id -> Nullable<Int4>,
    }
}

table! {
    spree_refund_reasons (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        mutable -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_refunds (id) {
        id -> Int4,
        payment_id -> Nullable<Int4>,
        amount -> Numeric,
        transaction_id -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_reimbursement_types (id) {
        id -> Int4,
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
        id -> Int4,
        name -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        mutable -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
        return_authorization_reason_id -> Nullable<Int4>,
    }
}

table! {
    spree_return_items (id) {
        id -> Int4,
        return_authorization_id -> Nullable<Int4>,
        inventory_unit_id -> Nullable<Int4>,
        exchange_variant_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        pre_tax_amount -> Numeric,
        included_tax_total -> Numeric,
        additional_tax_total -> Numeric,
        reception_status -> Nullable<Varchar>,
        acceptance_status -> Nullable<Varchar>,
        customer_return_id -> Nullable<Int4>,
        reimbursement_id -> Nullable<Int4>,
        acceptance_status_errors -> Nullable<Text>,
        preferred_reimbursement_type_id -> Nullable<Int4>,
        override_reimbursement_type_id -> Nullable<Int4>,
        resellable -> Bool,
    }
}

table! {
    spree_roles (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

table! {
    spree_role_users (id) {
        role_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        id -> Int8,
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
        address_id -> Nullable<Int4>,
        state -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        stock_location_id -> Nullable<Int4>,
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
        id -> Int4,
        name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_shipping_method_categories (id) {
        id -> Int4,
        shipping_method_id -> Int4,
        shipping_category_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_shipping_methods (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        display_on -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        tracking_url -> Nullable<Varchar>,
        admin_name -> Nullable<Varchar>,
        tax_category_id -> Nullable<Int4>,
        code -> Nullable<Varchar>,
    }
}

table! {
    spree_shipping_method_zones (id) {
        shipping_method_id -> Nullable<Int4>,
        zone_id -> Nullable<Int4>,
        id -> Int8,
    }
}

table! {
    spree_shipping_rates (id) {
        id -> Int4,
        shipment_id -> Nullable<Int4>,
        shipping_method_id -> Nullable<Int4>,
        selected -> Nullable<Bool>,
        cost -> Nullable<Numeric>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        tax_rate_id -> Nullable<Int4>,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_states (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        abbr -> Nullable<Varchar>,
        country_id -> Nullable<Int4>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_stock_items (id) {
        id -> Int4,
        stock_location_id -> Nullable<Int4>,
        variant_id -> Nullable<Int4>,
        count_on_hand -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        backorderable -> Nullable<Bool>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_stock_locations (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
    spree_stock_transfers (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        reference -> Nullable<Varchar>,
        source_location_id -> Nullable<Int4>,
        destination_location_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        number -> Nullable<Varchar>,
    }
}

table! {
    spree_store_credit_categories (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_store_credit_events (id) {
        id -> Int4,
        store_credit_id -> Int4,
        action -> Varchar,
        amount -> Nullable<Numeric>,
        authorization_code -> Varchar,
        user_total_amount -> Numeric,
        originator_id -> Nullable<Int4>,
        originator_type -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
        memo -> Nullable<Text>,
        deleted_at -> Nullable<Timestamp>,
        currency -> Nullable<Varchar>,
        amount_authorized -> Numeric,
        originator_id -> Nullable<Int4>,
        originator_type -> Nullable<Varchar>,
        type_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_store_credit_types (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        priority -> Nullable<Int4>,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_taggings (id) {
        id -> Int4,
        tag_id -> Nullable<Int4>,
        taggable_type -> Nullable<Varchar>,
        taggable_id -> Nullable<Int4>,
        tagger_type -> Nullable<Varchar>,
        tagger_id -> Nullable<Int4>,
        context -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    spree_tags (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        taggings_count -> Nullable<Int4>,
    }
}

table! {
    spree_tax_categories (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        is_default -> Nullable<Bool>,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        tax_code -> Nullable<Varchar>,
    }
}

table! {
    spree_taxonomies (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
        tax_category_id -> Nullable<Int4>,
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
        id -> Int4,
        analytics_id -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        engine -> Int4,
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
        updated_at -> Timestamp,
        discontinue_on -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

table! {
    spree_zone_members (id) {
        id -> Int4,
        zoneable_type -> Nullable<Varchar>,
        zoneable_id -> Nullable<Int4>,
        zone_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spree_zones (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        default_tax -> Nullable<Bool>,
        zone_members_count -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        kind -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    friendly_id_slugs,
    locales,
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
);
