table! {
    friendly_id_slugs (id) {
        id -> Integer,
        slug -> Varchar,
        sluggable_id -> Integer,
        sluggable_type -> Nullable<Varchar>,
        scope -> Nullable<Varchar>,
        created_at -> Nullable<Datetime>,
        deleted_at -> Nullable<Datetime>,
    }
}

table! {
    locales (id) {
        id -> Bigint,
        lang -> Varchar,
        code -> Varchar,
        message -> Text,
        updated_at -> Datetime,
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
    spree_addresses (id) {
        id -> Integer,
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
        state_id -> Nullable<Integer>,
        country_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    spree_adjustments (id) {
        id -> Integer,
        source_type -> Nullable<Varchar>,
        source_id -> Nullable<Integer>,
        adjustable_type -> Nullable<Varchar>,
        adjustable_id -> Nullable<Integer>,
        amount -> Nullable<Decimal>,
        label -> Nullable<Varchar>,
        mandatory -> Nullable<Bool>,
        eligible -> Nullable<Bool>,
        created_at -> Datetime,
        updated_at -> Datetime,
        state -> Nullable<Varchar>,
        order_id -> Integer,
        included -> Nullable<Bool>,
    }
}

table! {
    spree_assets (id) {
        id -> Integer,
        viewable_type -> Nullable<Varchar>,
        viewable_id -> Nullable<Integer>,
        attachment_width -> Nullable<Integer>,
        attachment_height -> Nullable<Integer>,
        attachment_file_size -> Nullable<Integer>,
        position -> Nullable<Integer>,
        attachment_content_type -> Nullable<Varchar>,
        attachment_file_name -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        attachment_updated_at -> Nullable<Datetime>,
        alt -> Nullable<Text>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    spree_calculators (id) {
        id -> Integer,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        calculable_type -> Nullable<Varchar>,
        calculable_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
        preferences -> Nullable<Text>,
        deleted_at -> Nullable<Datetime>,
    }
}

table! {
    spree_countries (id) {
        id -> Integer,
        iso_name -> Nullable<Varchar>,
        iso -> Nullable<Varchar>,
        iso3 -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        numcode -> Nullable<Integer>,
        states_required -> Nullable<Bool>,
        updated_at -> Nullable<Datetime>,
        zipcode_required -> Nullable<Bool>,
    }
}

table! {
    spree_credit_cards (id) {
        id -> Integer,
        month -> Nullable<Varchar>,
        year -> Nullable<Varchar>,
        cc_type -> Nullable<Varchar>,
        last_digits -> Nullable<Varchar>,
        address_id -> Nullable<Integer>,
        gateway_customer_profile_id -> Nullable<Varchar>,
        gateway_payment_profile_id -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
        name -> Nullable<Varchar>,
        user_id -> Nullable<Integer>,
        payment_method_id -> Nullable<Integer>,
        default -> Bool,
    }
}

table! {
    spree_customer_returns (id) {
        id -> Integer,
        number -> Nullable<Varchar>,
        stock_location_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    spree_gateways (id) {
        id -> Integer,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        active -> Nullable<Bool>,
        environment -> Nullable<Varchar>,
        server -> Nullable<Varchar>,
        test_mode -> Nullable<Bool>,
        created_at -> Datetime,
        updated_at -> Datetime,
        preferences -> Nullable<Text>,
    }
}

table! {
    spree_inventory_units (id) {
        id -> Integer,
        state -> Nullable<Varchar>,
        variant_id -> Nullable<Integer>,
        order_id -> Nullable<Integer>,
        shipment_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
        pending -> Nullable<Bool>,
        line_item_id -> Nullable<Integer>,
        quantity -> Nullable<Integer>,
        original_return_item_id -> Nullable<Integer>,
    }
}

table! {
    spree_line_items (id) {
        id -> Integer,
        variant_id -> Nullable<Integer>,
        order_id -> Nullable<Integer>,
        quantity -> Integer,
        price -> Decimal,
        created_at -> Datetime,
        updated_at -> Datetime,
        currency -> Nullable<Varchar>,
        cost_price -> Nullable<Decimal>,
        tax_category_id -> Nullable<Integer>,
        adjustment_total -> Nullable<Decimal>,
        additional_tax_total -> Nullable<Decimal>,
        promo_total -> Nullable<Decimal>,
        included_tax_total -> Decimal,
        pre_tax_amount -> Decimal,
        taxable_adjustment_total -> Decimal,
        non_taxable_adjustment_total -> Decimal,
    }
}

table! {
    spree_log_entries (id) {
        id -> Integer,
        source_type -> Nullable<Varchar>,
        source_id -> Nullable<Integer>,
        details -> Nullable<Text>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    spree_option_types (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        presentation -> Nullable<Varchar>,
        position -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    spree_option_type_prototypes (id) {
        prototype_id -> Nullable<Integer>,
        option_type_id -> Nullable<Integer>,
        id -> Bigint,
    }
}

table! {
    spree_option_values (id) {
        id -> Integer,
        position -> Nullable<Integer>,
        name -> Nullable<Varchar>,
        presentation -> Nullable<Varchar>,
        option_type_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    spree_option_value_variants (id) {
        variant_id -> Nullable<Integer>,
        option_value_id -> Nullable<Integer>,
        id -> Bigint,
    }
}

table! {
    spree_orders (id) {
        id -> Integer,
        number -> Nullable<Varchar>,
        item_total -> Decimal,
        total -> Decimal,
        state -> Nullable<Varchar>,
        adjustment_total -> Decimal,
        user_id -> Nullable<Integer>,
        completed_at -> Nullable<Datetime>,
        bill_address_id -> Nullable<Integer>,
        ship_address_id -> Nullable<Integer>,
        payment_total -> Nullable<Decimal>,
        shipment_state -> Nullable<Varchar>,
        payment_state -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        special_instructions -> Nullable<Text>,
        created_at -> Datetime,
        updated_at -> Datetime,
        currency -> Nullable<Varchar>,
        last_ip_address -> Nullable<Varchar>,
        created_by_id -> Nullable<Integer>,
        shipment_total -> Decimal,
        additional_tax_total -> Nullable<Decimal>,
        promo_total -> Nullable<Decimal>,
        channel -> Nullable<Varchar>,
        included_tax_total -> Decimal,
        item_count -> Nullable<Integer>,
        approver_id -> Nullable<Integer>,
        approved_at -> Nullable<Datetime>,
        confirmation_delivered -> Nullable<Bool>,
        considered_risky -> Nullable<Bool>,
        guest_token -> Nullable<Varchar>,
        canceled_at -> Nullable<Datetime>,
        canceler_id -> Nullable<Integer>,
        store_id -> Nullable<Integer>,
        state_lock_version -> Integer,
        taxable_adjustment_total -> Decimal,
        non_taxable_adjustment_total -> Decimal,
    }
}

table! {
    spree_order_promotions (id) {
        order_id -> Nullable<Integer>,
        promotion_id -> Nullable<Integer>,
        id -> Bigint,
    }
}

table! {
    spree_payments (id) {
        id -> Integer,
        amount -> Decimal,
        order_id -> Nullable<Integer>,
        source_type -> Nullable<Varchar>,
        source_id -> Nullable<Integer>,
        payment_method_id -> Nullable<Integer>,
        state -> Nullable<Varchar>,
        response_code -> Nullable<Varchar>,
        avs_response -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
        number -> Nullable<Varchar>,
        cvv_response_code -> Nullable<Varchar>,
        cvv_response_message -> Nullable<Varchar>,
    }
}

table! {
    spree_payment_capture_events (id) {
        id -> Integer,
        amount -> Nullable<Decimal>,
        payment_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    spree_payment_methods (id) {
        id -> Integer,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        active -> Nullable<Bool>,
        deleted_at -> Nullable<Datetime>,
        created_at -> Datetime,
        updated_at -> Datetime,
        display_on -> Nullable<Varchar>,
        auto_capture -> Nullable<Bool>,
        preferences -> Nullable<Text>,
        position -> Nullable<Integer>,
    }
}

table! {
    spree_preferences (id) {
        id -> Integer,
        value -> Nullable<Text>,
        key -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    spree_prices (id) {
        id -> Integer,
        variant_id -> Integer,
        amount -> Nullable<Decimal>,
        currency -> Nullable<Varchar>,
        deleted_at -> Nullable<Datetime>,
    }
}

table! {
    spree_products (id) {
        id -> Integer,
        name -> Varchar,
        description -> Nullable<Text>,
        available_on -> Nullable<Datetime>,
        discontinue_on -> Nullable<Datetime>,
        deleted_at -> Nullable<Datetime>,
        slug -> Nullable<Varchar>,
        meta_description -> Nullable<Text>,
        meta_keywords -> Nullable<Varchar>,
        tax_category_id -> Nullable<Integer>,
        shipping_category_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
        promotionable -> Nullable<Bool>,
        meta_title -> Nullable<Varchar>,
    }
}

table! {
    spree_products_taxons (id) {
        product_id -> Nullable<Integer>,
        taxon_id -> Nullable<Integer>,
        id -> Bigint,
        position -> Nullable<Integer>,
    }
}

table! {
    spree_product_option_types (id) {
        id -> Integer,
        position -> Nullable<Integer>,
        product_id -> Nullable<Integer>,
        option_type_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    spree_product_promotion_rules (id) {
        product_id -> Nullable<Integer>,
        promotion_rule_id -> Nullable<Integer>,
        id -> Bigint,
    }
}

table! {
    spree_product_properties (id) {
        id -> Integer,
        value -> Nullable<Varchar>,
        product_id -> Nullable<Integer>,
        property_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
        position -> Nullable<Integer>,
    }
}

table! {
    spree_promotions (id) {
        id -> Integer,
        description -> Nullable<Varchar>,
        expires_at -> Nullable<Datetime>,
        starts_at -> Nullable<Datetime>,
        name -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        usage_limit -> Nullable<Integer>,
        match_policy -> Nullable<Varchar>,
        code -> Nullable<Varchar>,
        advertise -> Nullable<Bool>,
        path -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
        promotion_category_id -> Nullable<Integer>,
    }
}

table! {
    spree_promotion_actions (id) {
        id -> Integer,
        promotion_id -> Nullable<Integer>,
        position -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        deleted_at -> Nullable<Datetime>,
    }
}

table! {
    spree_promotion_action_line_items (id) {
        id -> Integer,
        promotion_action_id -> Nullable<Integer>,
        variant_id -> Nullable<Integer>,
        quantity -> Nullable<Integer>,
    }
}

table! {
    spree_promotion_categories (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
        code -> Nullable<Varchar>,
    }
}

table! {
    spree_promotion_rules (id) {
        id -> Integer,
        promotion_id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        product_group_id -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
        code -> Nullable<Varchar>,
        preferences -> Nullable<Text>,
    }
}

table! {
    spree_promotion_rule_taxons (id) {
        id -> Integer,
        taxon_id -> Nullable<Integer>,
        promotion_rule_id -> Nullable<Integer>,
    }
}

table! {
    spree_promotion_rule_users (id) {
        user_id -> Nullable<Integer>,
        promotion_rule_id -> Nullable<Integer>,
        id -> Bigint,
    }
}

table! {
    spree_properties (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        presentation -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    spree_property_prototypes (id) {
        prototype_id -> Nullable<Integer>,
        property_id -> Nullable<Integer>,
        id -> Bigint,
    }
}

table! {
    spree_prototypes (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    spree_prototype_taxons (id) {
        id -> Integer,
        taxon_id -> Nullable<Integer>,
        prototype_id -> Nullable<Integer>,
    }
}

table! {
    spree_refunds (id) {
        id -> Integer,
        payment_id -> Nullable<Integer>,
        amount -> Decimal,
        transaction_id -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
        refund_reason_id -> Nullable<Integer>,
        reimbursement_id -> Nullable<Integer>,
    }
}

table! {
    spree_refund_reasons (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        mutable -> Nullable<Bool>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    spree_reimbursements (id) {
        id -> Integer,
        number -> Nullable<Varchar>,
        reimbursement_status -> Nullable<Varchar>,
        customer_return_id -> Nullable<Integer>,
        order_id -> Nullable<Integer>,
        total -> Nullable<Decimal>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    spree_reimbursement_credits (id) {
        id -> Integer,
        amount -> Decimal,
        reimbursement_id -> Nullable<Integer>,
        creditable_id -> Nullable<Integer>,
        creditable_type -> Nullable<Varchar>,
    }
}

table! {
    spree_reimbursement_types (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        mutable -> Nullable<Bool>,
        created_at -> Datetime,
        updated_at -> Datetime,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
    }
}

table! {
    spree_return_authorizations (id) {
        id -> Integer,
        number -> Nullable<Varchar>,
        state -> Nullable<Varchar>,
        order_id -> Nullable<Integer>,
        memo -> Nullable<Text>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
        stock_location_id -> Nullable<Integer>,
        return_authorization_reason_id -> Nullable<Integer>,
    }
}

table! {
    spree_return_authorization_reasons (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        mutable -> Nullable<Bool>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    spree_return_items (id) {
        id -> Integer,
        return_authorization_id -> Nullable<Integer>,
        inventory_unit_id -> Nullable<Integer>,
        exchange_variant_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
        pre_tax_amount -> Decimal,
        included_tax_total -> Decimal,
        additional_tax_total -> Decimal,
        reception_status -> Nullable<Varchar>,
        acceptance_status -> Nullable<Varchar>,
        customer_return_id -> Nullable<Integer>,
        reimbursement_id -> Nullable<Integer>,
        acceptance_status_errors -> Nullable<Text>,
        preferred_reimbursement_type_id -> Nullable<Integer>,
        override_reimbursement_type_id -> Nullable<Integer>,
        resellable -> Bool,
    }
}

table! {
    spree_roles (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
    }
}

table! {
    spree_role_users (id) {
        role_id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        id -> Bigint,
    }
}

table! {
    spree_shipments (id) {
        id -> Integer,
        tracking -> Nullable<Varchar>,
        number -> Nullable<Varchar>,
        cost -> Nullable<Decimal>,
        shipped_at -> Nullable<Datetime>,
        order_id -> Nullable<Integer>,
        address_id -> Nullable<Integer>,
        state -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
        stock_location_id -> Nullable<Integer>,
        adjustment_total -> Nullable<Decimal>,
        additional_tax_total -> Nullable<Decimal>,
        promo_total -> Nullable<Decimal>,
        included_tax_total -> Decimal,
        pre_tax_amount -> Decimal,
        taxable_adjustment_total -> Decimal,
        non_taxable_adjustment_total -> Decimal,
    }
}

table! {
    spree_shipping_categories (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    spree_shipping_methods (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        display_on -> Nullable<Varchar>,
        deleted_at -> Nullable<Datetime>,
        created_at -> Datetime,
        updated_at -> Datetime,
        tracking_url -> Nullable<Varchar>,
        admin_name -> Nullable<Varchar>,
        tax_category_id -> Nullable<Integer>,
        code -> Nullable<Varchar>,
    }
}

table! {
    spree_shipping_method_categories (id) {
        id -> Integer,
        shipping_method_id -> Integer,
        shipping_category_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    spree_shipping_method_zones (id) {
        shipping_method_id -> Nullable<Integer>,
        zone_id -> Nullable<Integer>,
        id -> Bigint,
    }
}

table! {
    spree_shipping_rates (id) {
        id -> Integer,
        shipment_id -> Nullable<Integer>,
        shipping_method_id -> Nullable<Integer>,
        selected -> Nullable<Bool>,
        cost -> Nullable<Decimal>,
        created_at -> Datetime,
        updated_at -> Datetime,
        tax_rate_id -> Nullable<Integer>,
    }
}

table! {
    spree_states (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        abbr -> Nullable<Varchar>,
        country_id -> Nullable<Integer>,
        updated_at -> Nullable<Datetime>,
    }
}

table! {
    spree_state_changes (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        previous_state -> Nullable<Varchar>,
        stateful_id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        stateful_type -> Nullable<Varchar>,
        next_state -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    spree_stock_items (id) {
        id -> Integer,
        stock_location_id -> Nullable<Integer>,
        variant_id -> Nullable<Integer>,
        count_on_hand -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
        backorderable -> Nullable<Bool>,
        deleted_at -> Nullable<Datetime>,
    }
}

table! {
    spree_stock_locations (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
        default -> Bool,
        address1 -> Nullable<Varchar>,
        address2 -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        state_id -> Nullable<Integer>,
        state_name -> Nullable<Varchar>,
        country_id -> Nullable<Integer>,
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
        id -> Integer,
        stock_item_id -> Nullable<Integer>,
        quantity -> Nullable<Integer>,
        action -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
        originator_type -> Nullable<Varchar>,
        originator_id -> Nullable<Integer>,
    }
}

table! {
    spree_stock_transfers (id) {
        id -> Integer,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        reference -> Nullable<Varchar>,
        source_location_id -> Nullable<Integer>,
        destination_location_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
        number -> Nullable<Varchar>,
    }
}

table! {
    spree_stores (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        url -> Nullable<Varchar>,
        meta_description -> Nullable<Text>,
        meta_keywords -> Nullable<Text>,
        seo_title -> Nullable<Varchar>,
        mail_from_address -> Nullable<Varchar>,
        default_currency -> Nullable<Varchar>,
        code -> Nullable<Varchar>,
        default -> Bool,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    spree_store_credits (id) {
        id -> Integer,
        user_id -> Nullable<Integer>,
        category_id -> Nullable<Integer>,
        created_by_id -> Nullable<Integer>,
        amount -> Decimal,
        amount_used -> Decimal,
        memo -> Nullable<Text>,
        deleted_at -> Nullable<Datetime>,
        currency -> Nullable<Varchar>,
        amount_authorized -> Decimal,
        originator_id -> Nullable<Integer>,
        originator_type -> Nullable<Varchar>,
        type_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    spree_store_credit_categories (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    spree_store_credit_events (id) {
        id -> Integer,
        store_credit_id -> Integer,
        action -> Varchar,
        amount -> Nullable<Decimal>,
        authorization_code -> Varchar,
        user_total_amount -> Decimal,
        originator_id -> Nullable<Integer>,
        originator_type -> Nullable<Varchar>,
        deleted_at -> Nullable<Datetime>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    spree_store_credit_types (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        priority -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

table! {
    spree_taggings (id) {
        id -> Integer,
        tag_id -> Nullable<Integer>,
        taggable_type -> Nullable<Varchar>,
        taggable_id -> Nullable<Integer>,
        tagger_type -> Nullable<Varchar>,
        tagger_id -> Nullable<Integer>,
        context -> Nullable<Varchar>,
        created_at -> Nullable<Datetime>,
    }
}

table! {
    spree_tags (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        taggings_count -> Nullable<Integer>,
    }
}

table! {
    spree_taxonomies (id) {
        id -> Integer,
        name -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
        position -> Nullable<Integer>,
    }
}

table! {
    spree_taxons (id) {
        id -> Integer,
        parent_id -> Nullable<Integer>,
        position -> Nullable<Integer>,
        name -> Varchar,
        permalink -> Nullable<Varchar>,
        taxonomy_id -> Nullable<Integer>,
        lft -> Nullable<Integer>,
        rgt -> Nullable<Integer>,
        icon_file_name -> Nullable<Varchar>,
        icon_content_type -> Nullable<Varchar>,
        icon_file_size -> Nullable<Integer>,
        icon_updated_at -> Nullable<Datetime>,
        description -> Nullable<Text>,
        created_at -> Datetime,
        updated_at -> Datetime,
        meta_title -> Nullable<Varchar>,
        meta_description -> Nullable<Varchar>,
        meta_keywords -> Nullable<Varchar>,
        depth -> Nullable<Integer>,
    }
}

table! {
    spree_tax_categories (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        is_default -> Nullable<Bool>,
        deleted_at -> Nullable<Datetime>,
        created_at -> Datetime,
        updated_at -> Datetime,
        tax_code -> Nullable<Varchar>,
    }
}

table! {
    spree_tax_rates (id) {
        id -> Integer,
        amount -> Nullable<Decimal>,
        zone_id -> Nullable<Integer>,
        tax_category_id -> Nullable<Integer>,
        included_in_price -> Nullable<Bool>,
        created_at -> Datetime,
        updated_at -> Datetime,
        name -> Nullable<Varchar>,
        show_rate_in_label -> Nullable<Bool>,
        deleted_at -> Nullable<Datetime>,
    }
}

table! {
    spree_trackers (id) {
        id -> Integer,
        analytics_id -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        created_at -> Datetime,
        updated_at -> Datetime,
        engine -> Integer,
    }
}

table! {
    spree_users (id) {
        id -> Integer,
        encrypted_password -> Nullable<Varchar>,
        password_salt -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        remember_token -> Nullable<Varchar>,
        persistence_token -> Nullable<Varchar>,
        reset_password_token -> Nullable<Varchar>,
        perishable_token -> Nullable<Varchar>,
        sign_in_count -> Integer,
        failed_attempts -> Integer,
        last_request_at -> Nullable<Datetime>,
        current_sign_in_at -> Nullable<Datetime>,
        last_sign_in_at -> Nullable<Datetime>,
        current_sign_in_ip -> Nullable<Varchar>,
        last_sign_in_ip -> Nullable<Varchar>,
        login -> Nullable<Varchar>,
        ship_address_id -> Nullable<Integer>,
        bill_address_id -> Nullable<Integer>,
        authentication_token -> Nullable<Varchar>,
        unlock_token -> Nullable<Varchar>,
        locked_at -> Nullable<Datetime>,
        reset_password_sent_at -> Nullable<Datetime>,
        created_at -> Datetime,
        updated_at -> Datetime,
        spree_api_key -> Nullable<Varchar>,
        remember_created_at -> Nullable<Datetime>,
        deleted_at -> Nullable<Datetime>,
        confirmation_token -> Nullable<Varchar>,
        confirmed_at -> Nullable<Datetime>,
        confirmation_sent_at -> Nullable<Datetime>,
    }
}

table! {
    spree_variants (id) {
        id -> Integer,
        sku -> Varchar,
        weight -> Nullable<Decimal>,
        height -> Nullable<Decimal>,
        width -> Nullable<Decimal>,
        depth -> Nullable<Decimal>,
        deleted_at -> Nullable<Datetime>,
        discontinue_on -> Nullable<Datetime>,
        is_master -> Nullable<Bool>,
        product_id -> Nullable<Integer>,
        cost_price -> Nullable<Decimal>,
        cost_currency -> Nullable<Varchar>,
        position -> Nullable<Integer>,
        track_inventory -> Nullable<Bool>,
        tax_category_id -> Nullable<Integer>,
        updated_at -> Datetime,
        created_at -> Datetime,
    }
}

table! {
    spree_zones (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        default_tax -> Nullable<Bool>,
        zone_members_count -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
        kind -> Nullable<Varchar>,
    }
}

table! {
    spree_zone_members (id) {
        id -> Integer,
        zoneable_type -> Nullable<Varchar>,
        zoneable_id -> Nullable<Integer>,
        zone_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
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
    spree_option_types,
    spree_option_type_prototypes,
    spree_option_values,
    spree_option_value_variants,
    spree_orders,
    spree_order_promotions,
    spree_payments,
    spree_payment_capture_events,
    spree_payment_methods,
    spree_preferences,
    spree_prices,
    spree_products,
    spree_products_taxons,
    spree_product_option_types,
    spree_product_promotion_rules,
    spree_product_properties,
    spree_promotions,
    spree_promotion_actions,
    spree_promotion_action_line_items,
    spree_promotion_categories,
    spree_promotion_rules,
    spree_promotion_rule_taxons,
    spree_promotion_rule_users,
    spree_properties,
    spree_property_prototypes,
    spree_prototypes,
    spree_prototype_taxons,
    spree_refunds,
    spree_refund_reasons,
    spree_reimbursements,
    spree_reimbursement_credits,
    spree_reimbursement_types,
    spree_return_authorizations,
    spree_return_authorization_reasons,
    spree_return_items,
    spree_roles,
    spree_role_users,
    spree_shipments,
    spree_shipping_categories,
    spree_shipping_methods,
    spree_shipping_method_categories,
    spree_shipping_method_zones,
    spree_shipping_rates,
    spree_states,
    spree_state_changes,
    spree_stock_items,
    spree_stock_locations,
    spree_stock_movements,
    spree_stock_transfers,
    spree_stores,
    spree_store_credits,
    spree_store_credit_categories,
    spree_store_credit_events,
    spree_store_credit_types,
    spree_taggings,
    spree_tags,
    spree_taxonomies,
    spree_taxons,
    spree_tax_categories,
    spree_tax_rates,
    spree_trackers,
    spree_users,
    spree_variants,
    spree_zones,
    spree_zone_members,
);
