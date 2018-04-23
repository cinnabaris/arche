CREATE TABLE friendly_id_slugs (
    id BIGSERIAL PRIMARY KEY,
    slug character varying NOT NULL,
    sluggable_id BIGINT NOT NULL,
    sluggable_type character varying(50),
    scope character varying,
    created_at timestamp without time zone,
    deleted_at timestamp without time zone
);

CREATE TABLE spree_addresses (
    id BIGSERIAL PRIMARY KEY,
    firstname character varying,
    lastname character varying,
    address1 character varying,
    address2 character varying,
    city character varying,
    zipcode character varying,
    phone character varying,
    state_name character varying,
    alternative_phone character varying,
    company character varying,
    state_id BIGINT,
    country_id BIGINT,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_adjustments (
    id BIGSERIAL PRIMARY KEY,
    source_type character varying,
    source_id BIGINT,
    adjustable_type character varying,
    adjustable_id BIGINT,
    amount numeric(10,2),
    label character varying,
    mandatory boolean,
    eligible boolean DEFAULT true,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    state character varying,
    order_id BIGINT NOT NULL,
    included boolean DEFAULT false
);

CREATE TABLE spree_assets (
    id BIGSERIAL PRIMARY KEY,
    viewable_type character varying,
    viewable_id BIGINT,
    attachment_width integer,
    attachment_height integer,
    attachment_file_size integer,
    "position" integer,
    attachment_content_type character varying,
    attachment_file_name character varying,
    type character varying(75),
    attachment_updated_at timestamp without time zone,
    alt text,
    created_at timestamp without time zone,
    updated_at timestamp without time zone
);

CREATE TABLE spree_calculators (
    id BIGSERIAL PRIMARY KEY,
    type character varying,
    calculable_type character varying,
    calculable_id BIGINT,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    preferences text,
    deleted_at timestamp without time zone
);

CREATE TABLE spree_countries (
    id BIGSERIAL PRIMARY KEY,
    iso_name character varying,
    iso character varying,
    iso3 character varying,
    name character varying,
    numcode integer,
    states_required boolean DEFAULT false,
    updated_at timestamp without time zone,
    zipcode_required boolean DEFAULT true
);

CREATE TABLE spree_credit_cards (
    id BIGSERIAL PRIMARY KEY,
    month character varying,
    year character varying,
    cc_type character varying,
    last_digits character varying,
    address_id BIGINT,
    gateway_customer_profile_id character varying,
    gateway_payment_profile_id character varying,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    name character varying,
    user_id BIGINT,
    payment_method_id BIGINT,
    "default" boolean DEFAULT false NOT NULL
);

CREATE TABLE spree_customer_returns (
    id BIGSERIAL PRIMARY KEY,
    number character varying,
    stock_location_id BIGINT,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_gateways (
    id BIGSERIAL PRIMARY KEY,
    type character varying,
    name character varying,
    description text,
    active boolean DEFAULT true,
    environment character varying DEFAULT 'development',
    server character varying DEFAULT 'test',
    test_mode boolean DEFAULT true,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    preferences text
);

CREATE TABLE spree_inventory_units (
    id BIGSERIAL PRIMARY KEY,
    state character varying,
    variant_id BIGINT,
    order_id BIGINT,
    shipment_id BIGINT,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    pending boolean DEFAULT true,
    line_item_id BIGINT,
    quantity integer DEFAULT 1,
    original_return_item_id BIGINT
);

CREATE TABLE spree_line_items (
    id BIGSERIAL PRIMARY KEY,
    variant_id BIGINT,
    order_id BIGINT,
    quantity integer NOT NULL,
    price numeric(10,2) NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    currency character varying,
    cost_price numeric(10,2),
    tax_category_id BIGINT,
    adjustment_total numeric(10,2) DEFAULT 0.0,
    additional_tax_total numeric(10,2) DEFAULT 0.0,
    promo_total numeric(10,2) DEFAULT 0.0,
    included_tax_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    pre_tax_amount numeric(12,4) DEFAULT 0.0 NOT NULL,
    taxable_adjustment_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    non_taxable_adjustment_total numeric(10,2) DEFAULT 0.0 NOT NULL
);

CREATE TABLE spree_log_entries (
    id BIGSERIAL PRIMARY KEY,
    source_type character varying,
    source_id BIGINT,
    details text,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_option_type_prototypes (
    id BIGSERIAL PRIMARY KEY,
    prototype_id BIGINT,
    option_type_id BIGINT
);

CREATE TABLE spree_option_types (
    id BIGSERIAL PRIMARY KEY,
    name character varying(100),
    presentation character varying(100),
    "position" integer DEFAULT 0 NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_option_value_variants (
    id BIGSERIAL PRIMARY KEY,
    variant_id BIGINT,
    option_value_id BIGINT
);

CREATE TABLE spree_option_values (
    id BIGSERIAL PRIMARY KEY,
    "position" integer,
    name character varying,
    presentation character varying,
    option_type_id BIGINT,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_order_promotions (
    id BIGSERIAL PRIMARY KEY,
    order_id BIGINT,
    promotion_id BIGINT
);

CREATE TABLE spree_orders (
    id BIGSERIAL PRIMARY KEY,
    number character varying(32),
    item_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    total numeric(10,2) DEFAULT 0.0 NOT NULL,
    state character varying,
    adjustment_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    user_id BIGINT,
    completed_at timestamp without time zone,
    bill_address_id BIGINT,
    ship_address_id BIGINT,
    payment_total numeric(10,2) DEFAULT 0.0,
    shipment_state character varying,
    payment_state character varying,
    email character varying,
    special_instructions text,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    currency character varying,
    last_ip_address character varying,
    created_by_id BIGINT,
    shipment_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    additional_tax_total numeric(10,2) DEFAULT 0.0,
    promo_total numeric(10,2) DEFAULT 0.0,
    channel character varying DEFAULT 'spree',
    included_tax_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    item_count integer DEFAULT 0,
    approver_id BIGINT,
    approved_at timestamp without time zone,
    confirmation_delivered boolean DEFAULT false,
    considered_risky boolean DEFAULT false,
    guest_token character varying,
    canceled_at timestamp without time zone,
    canceler_id BIGINT,
    store_id BIGINT,
    state_lock_version integer DEFAULT 0 NOT NULL,
    taxable_adjustment_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    non_taxable_adjustment_total numeric(10,2) DEFAULT 0.0 NOT NULL
);

CREATE TABLE spree_payment_capture_events (
    id BIGSERIAL PRIMARY KEY,
    amount numeric(10,2) DEFAULT 0.0,
    payment_id BIGINT,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_payment_methods (
    id BIGSERIAL PRIMARY KEY,
    type character varying,
    name character varying,
    description text,
    active boolean DEFAULT true,
    deleted_at timestamp without time zone,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    display_on character varying DEFAULT 'both',
    auto_capture boolean,
    preferences text,
    "position" integer DEFAULT 0
);

CREATE TABLE spree_payments (
    id BIGSERIAL PRIMARY KEY,
    amount numeric(10,2) DEFAULT 0.0 NOT NULL,
    order_id BIGINT,
    source_type character varying,
    source_id BIGINT,
    payment_method_id BIGINT,
    state character varying,
    response_code character varying,
    avs_response character varying,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    number character varying,
    cvv_response_code character varying,
    cvv_response_message character varying
);

CREATE TABLE spree_preferences (
    id BIGSERIAL PRIMARY KEY,
    value text,
    key character varying,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_prices (
    id BIGSERIAL PRIMARY KEY,
    variant_id BIGINT NOT NULL,
    amount numeric(10,2),
    currency character varying,
    deleted_at timestamp without time zone
);

CREATE TABLE spree_product_option_types (
    id BIGSERIAL PRIMARY KEY,
    "position" integer,
    product_id BIGINT,
    option_type_id BIGINT,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_product_promotion_rules (
    id BIGSERIAL PRIMARY KEY,
    product_id BIGINT,
    promotion_rule_id BIGINT
);

CREATE TABLE spree_product_properties (
    id BIGSERIAL PRIMARY KEY,
    value character varying,
    product_id BIGINT,
    property_id BIGINT,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    "position" integer DEFAULT 0
);

CREATE TABLE spree_products (
    id BIGSERIAL PRIMARY KEY,
    name character varying DEFAULT '' NOT NULL,
    description text,
    available_on timestamp without time zone,
    deleted_at timestamp without time zone,
    slug character varying,
    meta_description text,
    meta_keywords character varying,
    tax_category_id BIGINT,
    shipping_category_id BIGINT,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    promotionable boolean DEFAULT true,
    meta_title character varying,
    discontinue_on timestamp without time zone
);

CREATE TABLE spree_products_taxons (
    id BIGSERIAL PRIMARY KEY,
    product_id BIGINT,
    taxon_id BIGINT,
    "position" integer
);

CREATE TABLE spree_promotion_action_line_items (
    id BIGSERIAL PRIMARY KEY,
    promotion_action_id BIGINT,
    variant_id BIGINT,
    quantity integer DEFAULT 1
);

CREATE TABLE spree_promotion_actions (
    id BIGSERIAL PRIMARY KEY,
    promotion_id BIGINT,
    "position" integer,
    type character varying,
    deleted_at timestamp without time zone
);

CREATE TABLE spree_promotion_categories (
    id BIGSERIAL PRIMARY KEY,
    name character varying,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    code character varying
);

CREATE TABLE spree_promotion_rule_taxons (
    id BIGSERIAL PRIMARY KEY,
    taxon_id BIGINT,
    promotion_rule_id BIGINT
);

CREATE TABLE spree_promotion_rule_users (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT,
    promotion_rule_id BIGINT
);

CREATE TABLE spree_promotion_rules (
    id BIGSERIAL PRIMARY KEY,
    promotion_id BIGINT,
    user_id BIGINT,
    product_group_id BIGINT,
    type character varying,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    code character varying,
    preferences text
);

CREATE TABLE spree_promotions (
    id BIGSERIAL PRIMARY KEY,
    description character varying,
    expires_at timestamp without time zone,
    starts_at timestamp without time zone,
    name character varying,
    type character varying,
    usage_limit integer,
    match_policy character varying DEFAULT 'all',
    code character varying,
    advertise boolean DEFAULT false,
    path character varying,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    promotion_category_id BIGINT
);

CREATE TABLE spree_properties (
    id BIGSERIAL PRIMARY KEY,
    name character varying,
    presentation character varying NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_property_prototypes (
    id BIGSERIAL PRIMARY KEY,
    prototype_id BIGINT,
    property_id BIGINT
);

CREATE TABLE spree_prototype_taxons (
    id BIGSERIAL PRIMARY KEY,
    taxon_id BIGINT,
    prototype_id BIGINT
);

CREATE TABLE spree_prototypes (
    id BIGSERIAL PRIMARY KEY,
    name character varying,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_refund_reasons (
    id BIGSERIAL PRIMARY KEY,
    name character varying,
    active boolean DEFAULT true,
    mutable boolean DEFAULT true,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_refunds (
    id BIGSERIAL PRIMARY KEY,
    payment_id BIGINT,
    amount numeric(10,2) DEFAULT 0.0 NOT NULL,
    transaction_id character varying,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    refund_reason_id BIGINT,
    reimbursement_id BIGINT
);

CREATE TABLE spree_reimbursement_credits (
    id BIGSERIAL PRIMARY KEY,
    amount numeric(10,2) DEFAULT 0.0 NOT NULL,
    reimbursement_id BIGINT,
    creditable_id BIGINT,
    creditable_type character varying
);

CREATE TABLE spree_reimbursement_types (
    id BIGSERIAL PRIMARY KEY,
    name character varying,
    active boolean DEFAULT true,
    mutable boolean DEFAULT true,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    type character varying
);

CREATE TABLE spree_reimbursements (
    id BIGSERIAL PRIMARY KEY,
    number character varying,
    reimbursement_status character varying,
    customer_return_id BIGINT,
    order_id BIGINT,
    total numeric(10,2),
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_return_authorization_reasons (
    id BIGSERIAL PRIMARY KEY,
    name character varying,
    active boolean DEFAULT true,
    mutable boolean DEFAULT true,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_return_authorizations (
    id BIGSERIAL PRIMARY KEY,
    number character varying,
    state character varying,
    order_id BIGINT,
    memo text,
    created_at timestamp without time zone,
    updated_at timestamp without time zone,
    stock_location_id BIGINT,
    return_authorization_reason_id BIGINT
);

CREATE TABLE spree_return_items (
    id BIGSERIAL PRIMARY KEY,
    return_authorization_id BIGINT,
    inventory_unit_id BIGINT,
    exchange_variant_id BIGINT,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    pre_tax_amount numeric(12,4) DEFAULT 0.0 NOT NULL,
    included_tax_total numeric(12,4) DEFAULT 0.0 NOT NULL,
    additional_tax_total numeric(12,4) DEFAULT 0.0 NOT NULL,
    reception_status character varying,
    acceptance_status character varying,
    customer_return_id BIGINT,
    reimbursement_id BIGINT,
    acceptance_status_errors text,
    preferred_reimbursement_type_id BIGINT,
    override_reimbursement_type_id BIGINT,
    resellable boolean DEFAULT true NOT NULL
);

CREATE TABLE spree_role_users (
    id BIGSERIAL PRIMARY KEY,
    role_id BIGINT,
    user_id BIGINT
);

CREATE TABLE spree_roles (
    id BIGSERIAL PRIMARY KEY,
    name character varying
);

CREATE TABLE spree_shipments (
    id BIGSERIAL PRIMARY KEY,
    tracking character varying,
    number character varying,
    cost numeric(10,2) DEFAULT 0.0,
    shipped_at timestamp without time zone,
    order_id BIGINT,
    address_id BIGINT,
    state character varying,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    stock_location_id BIGINT,
    adjustment_total numeric(10,2) DEFAULT 0.0,
    additional_tax_total numeric(10,2) DEFAULT 0.0,
    promo_total numeric(10,2) DEFAULT 0.0,
    included_tax_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    pre_tax_amount numeric(12,4) DEFAULT 0.0 NOT NULL,
    taxable_adjustment_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    non_taxable_adjustment_total numeric(10,2) DEFAULT 0.0 NOT NULL
);

CREATE TABLE spree_shipping_categories (
    id BIGSERIAL PRIMARY KEY,
    name character varying,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_shipping_method_categories (
    id BIGSERIAL PRIMARY KEY,
    shipping_method_id BIGINT NOT NULL,
    shipping_category_id BIGINT NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_shipping_method_zones (
    id BIGSERIAL PRIMARY KEY,
    shipping_method_id BIGINT,
    zone_id BIGINT
);

CREATE TABLE spree_shipping_methods (
    id BIGSERIAL PRIMARY KEY,
    name character varying,
    display_on character varying,
    deleted_at timestamp without time zone,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    tracking_url character varying,
    admin_name character varying,
    tax_category_id BIGINT,
    code character varying
);

CREATE TABLE spree_shipping_rates (
    id BIGSERIAL PRIMARY KEY,
    shipment_id BIGINT,
    shipping_method_id BIGINT,
    selected boolean DEFAULT false,
    cost numeric(8,2) DEFAULT 0,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    tax_rate_id BIGINT
);

CREATE TABLE spree_state_changes (
    id BIGSERIAL PRIMARY KEY,
    name character varying,
    previous_state character varying,
    stateful_id BIGINT,
    user_id BIGINT,
    stateful_type character varying,
    next_state character varying,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_states (
    id BIGSERIAL PRIMARY KEY,
    name character varying,
    abbr character varying,
    country_id BIGINT,
    updated_at timestamp without time zone
);

CREATE TABLE spree_stock_items (
    id BIGSERIAL PRIMARY KEY,
    stock_location_id BIGINT,
    variant_id BIGINT,
    count_on_hand integer DEFAULT 0 NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    backorderable boolean DEFAULT false,
    deleted_at timestamp without time zone
);

CREATE TABLE spree_stock_locations (
    id BIGSERIAL PRIMARY KEY,
    name character varying,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    "default" boolean DEFAULT false NOT NULL,
    address1 character varying,
    address2 character varying,
    city character varying,
    state_id BIGINT,
    state_name character varying,
    country_id BIGINT,
    zipcode character varying,
    phone character varying,
    active boolean DEFAULT true,
    backorderable_default boolean DEFAULT false,
    propagate_all_variants boolean DEFAULT true,
    admin_name character varying
);

CREATE TABLE spree_stock_movements (
    id BIGSERIAL PRIMARY KEY,
    stock_item_id BIGINT,
    quantity integer DEFAULT 0,
    action character varying,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    originator_type character varying,
    originator_id BIGINT
);

CREATE TABLE spree_stock_transfers (
    id BIGSERIAL PRIMARY KEY,
    type character varying,
    reference character varying,
    source_location_id BIGINT,
    destination_location_id BIGINT,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    number character varying
);

CREATE TABLE spree_store_credit_categories (
    id BIGSERIAL PRIMARY KEY,
    name character varying,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_store_credit_events (
    id BIGSERIAL PRIMARY KEY,
    store_credit_id BIGINT NOT NULL,
    action character varying NOT NULL,
    amount numeric(8,2),
    authorization_code character varying NOT NULL,
    user_total_amount numeric(8,2) DEFAULT 0.0 NOT NULL,
    originator_id BIGINT,
    originator_type character varying,
    deleted_at timestamp without time zone,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_store_credit_types (
    id BIGSERIAL PRIMARY KEY,
    name character varying,
    priority integer,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_store_credits (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT,
    category_id BIGINT,
    created_by_id BIGINT,
    amount numeric(8,2) DEFAULT 0.0 NOT NULL,
    amount_used numeric(8,2) DEFAULT 0.0 NOT NULL,
    memo text,
    deleted_at timestamp without time zone,
    currency character varying,
    amount_authorized numeric(8,2) DEFAULT 0.0 NOT NULL,
    originator_id BIGINT,
    originator_type character varying,
    type_id BIGINT,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_stores (
    id BIGSERIAL PRIMARY KEY,
    name character varying,
    url character varying,
    meta_description text,
    meta_keywords text,
    seo_title character varying,
    mail_from_address character varying,
    default_currency character varying,
    code character varying,
    "default" boolean DEFAULT false NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_taggings (
    id BIGSERIAL PRIMARY KEY,
    tag_id BIGINT,
    taggable_type character varying,
    taggable_id BIGINT,
    tagger_type character varying,
    tagger_id BIGINT,
    context character varying(128),
    created_at timestamp without time zone
);

CREATE TABLE spree_tags (
    id BIGSERIAL PRIMARY KEY,
    name character varying,
    taggings_count integer DEFAULT 0
);

CREATE TABLE spree_tax_categories (
    id BIGSERIAL PRIMARY KEY,
    name character varying,
    description character varying,
    is_default boolean DEFAULT false,
    deleted_at timestamp without time zone,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    tax_code character varying
);

CREATE TABLE spree_tax_rates (
    id BIGSERIAL PRIMARY KEY,
    amount numeric(8,5),
    zone_id BIGINT,
    tax_category_id BIGINT,
    included_in_price boolean DEFAULT false,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    name character varying,
    show_rate_in_label boolean DEFAULT true,
    deleted_at timestamp without time zone
);

CREATE TABLE spree_taxonomies (
    id BIGSERIAL PRIMARY KEY,
    name character varying NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    "position" integer DEFAULT 0
);

CREATE TABLE spree_taxons (
    id BIGSERIAL PRIMARY KEY,
    parent_id BIGINT,
    "position" integer DEFAULT 0,
    name character varying NOT NULL,
    permalink character varying,
    taxonomy_id BIGINT,
    lft integer,
    rgt integer,
    icon_file_name character varying,
    icon_content_type character varying,
    icon_file_size integer,
    icon_updated_at timestamp without time zone,
    description text,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    meta_title character varying,
    meta_description character varying,
    meta_keywords character varying,
    depth integer
);

CREATE TABLE spree_trackers (
    id BIGSERIAL PRIMARY KEY,
    analytics_id character varying,
    active boolean DEFAULT true,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    engine integer DEFAULT 0 NOT NULL
);

CREATE TABLE spree_users (
    id BIGSERIAL PRIMARY KEY,
    encrypted_password character varying(128),
    password_salt character varying(128),
    email character varying,
    remember_token character varying,
    persistence_token character varying,
    reset_password_token character varying,
    perishable_token character varying,
    sign_in_count integer DEFAULT 0 NOT NULL,
    failed_attempts integer DEFAULT 0 NOT NULL,
    last_request_at timestamp without time zone,
    current_sign_in_at timestamp without time zone,
    last_sign_in_at timestamp without time zone,
    current_sign_in_ip character varying,
    last_sign_in_ip character varying,
    login character varying,
    ship_address_id BIGINT,
    bill_address_id BIGINT,
    authentication_token character varying,
    unlock_token character varying,
    locked_at timestamp without time zone,
    reset_password_sent_at timestamp without time zone,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    spree_api_key character varying(48),
    remember_created_at timestamp without time zone,
    deleted_at timestamp without time zone,
    confirmation_token character varying,
    confirmed_at timestamp without time zone,
    confirmation_sent_at timestamp without time zone
);

CREATE TABLE spree_variants (
    id BIGSERIAL PRIMARY KEY,
    sku character varying DEFAULT '' NOT NULL,
    weight numeric(8,2) DEFAULT 0.0,
    height numeric(8,2),
    width numeric(8,2),
    depth numeric(8,2),
    deleted_at timestamp without time zone,
    is_master boolean DEFAULT false,
    product_id BIGINT,
    cost_price numeric(10,2),
    "position" integer,
    cost_currency character varying,
    track_inventory boolean DEFAULT true,
    tax_category_id BIGINT,
    updated_at timestamp without time zone NOT NULL,
    discontinue_on timestamp without time zone,
    created_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_zone_members (
    id BIGSERIAL PRIMARY KEY,
    zoneable_type character varying,
    zoneable_id BIGINT,
    zone_id BIGINT,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_zones (
    id BIGSERIAL PRIMARY KEY,
    name character varying,
    description character varying,
    default_tax boolean DEFAULT false,
    zone_members_count integer DEFAULT 0,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    kind character varying
);


CREATE UNIQUE INDEX idx_spree_users_on_email ON spree_users USING btree (email);
CREATE INDEX idx_spree_addresses_on_firstname ON spree_addresses USING btree (firstname);
CREATE INDEX idx_spree_addresses_on_lastname ON spree_addresses USING btree (lastname);
CREATE INDEX idx_spree_assets_on_viewable_id ON spree_assets USING btree (viewable_id);
CREATE INDEX idx_spree_assets_on_viewable_type_and_type ON spree_assets USING btree (viewable_type, type);

CREATE INDEX idx_friendly_id_slugs_on_deleted_at ON friendly_id_slugs USING btree (deleted_at);
CREATE INDEX idx_friendly_id_slugs_on_slug_and_sluggable_type ON friendly_id_slugs USING btree (slug, sluggable_type);
CREATE UNIQUE INDEX idx_friendly_id_slugs_on_slug_and_sluggable_type_and_scope ON friendly_id_slugs USING btree (slug, sluggable_type, scope);
CREATE INDEX idx_friendly_id_slugs_on_sluggable_id ON friendly_id_slugs USING btree (sluggable_id);
CREATE INDEX idx_friendly_id_slugs_on_sluggable_type ON friendly_id_slugs USING btree (sluggable_type);

CREATE INDEX idx_spree_inventory_units_on_order_id ON spree_inventory_units USING btree (order_id);
CREATE INDEX idx_spree_inventory_units_on_shipment_id ON spree_inventory_units USING btree (shipment_id);
CREATE INDEX idx_spree_inventory_units_on_variant_id ON spree_inventory_units USING btree (variant_id);
CREATE UNIQUE INDEX idx_spree_option_values_variants_on_variant_id_and_option_value_id ON spree_option_value_variants USING btree (variant_id, option_value_id);
CREATE INDEX idx_spree_product_properties_on_product_id ON spree_product_properties USING btree (product_id);
CREATE INDEX idx_spree_products_promotion_rules_on_product_id ON spree_product_promotion_rules USING btree (product_id);
CREATE INDEX idx_spree_products_promotion_rules_on_promotion_rule_and_product ON spree_product_promotion_rules USING btree (promotion_rule_id, product_id);
CREATE INDEX idx_spree_promotion_rules_on_product_group_id ON spree_promotion_rules USING btree (product_group_id);
CREATE INDEX idx_spree_promotion_rules_on_user_id ON spree_promotion_rules USING btree (user_id);
CREATE INDEX idx_spree_promotion_rules_users_on_promotion_rule_id ON spree_promotion_rule_users USING btree (promotion_rule_id);
CREATE INDEX idx_spree_promotion_rules_users_on_user_id_and_promotion_rule_id ON spree_promotion_rule_users USING btree (user_id, promotion_rule_id);
CREATE UNIQUE INDEX idx_spree_property_prototypes_on_prototype_id_and_property_id ON spree_property_prototypes USING btree (prototype_id, property_id);
CREATE INDEX idx_spree_refunds_on_refund_reason_id ON spree_refunds USING btree (refund_reason_id);
CREATE INDEX idx_spree_reimbursement_credits_on_creditable_id_and_type ON spree_reimbursement_credits USING btree (creditable_id, creditable_type);
CREATE INDEX idx_spree_return_authorizations_on_return_authorization_reason_id ON spree_return_authorizations USING btree (return_authorization_reason_id);
CREATE INDEX idx_spree_return_items_on_customer_return_id ON spree_return_items USING btree (customer_return_id);
CREATE INDEX idx_spree_addresses_on_country_id ON spree_addresses USING btree (country_id);
CREATE INDEX idx_spree_addresses_on_state_id ON spree_addresses USING btree (state_id);
CREATE INDEX idx_spree_adjustments_on_adjustable_id_and_adjustable_type ON spree_adjustments USING btree (adjustable_id, adjustable_type);
CREATE INDEX idx_spree_adjustments_on_eligible ON spree_adjustments USING btree (eligible);
CREATE INDEX idx_spree_adjustments_on_order_id ON spree_adjustments USING btree (order_id);
CREATE INDEX idx_spree_adjustments_on_source_id_and_source_type ON spree_adjustments USING btree (source_id, source_type);
CREATE INDEX idx_spree_assets_on_position ON spree_assets USING btree ("position");
CREATE INDEX idx_spree_calculators_on_calculable_id_and_calculable_type ON spree_calculators USING btree (calculable_id, calculable_type);
CREATE INDEX idx_spree_calculators_on_deleted_at ON spree_calculators USING btree (deleted_at);
CREATE INDEX idx_spree_calculators_on_id_and_type ON spree_calculators USING btree (id, type);
CREATE UNIQUE INDEX idx_spree_countries_on_lower_iso_name ON spree_countries USING btree (lower((iso_name)::text));
CREATE UNIQUE INDEX idx_spree_countries_on_lower_name ON spree_countries USING btree (lower((name)::text));
CREATE INDEX idx_spree_credit_cards_on_address_id ON spree_credit_cards USING btree (address_id);
CREATE INDEX idx_spree_credit_cards_on_payment_method_id ON spree_credit_cards USING btree (payment_method_id);
CREATE INDEX idx_spree_credit_cards_on_user_id ON spree_credit_cards USING btree (user_id);
CREATE UNIQUE INDEX idx_spree_customer_returns_on_number ON spree_customer_returns USING btree (number);
CREATE INDEX idx_spree_customer_returns_on_stock_location_id ON spree_customer_returns USING btree (stock_location_id);
CREATE INDEX idx_spree_gateways_on_active ON spree_gateways USING btree (active);
CREATE INDEX idx_spree_gateways_on_test_mode ON spree_gateways USING btree (test_mode);
CREATE INDEX idx_spree_inventory_units_on_line_item_id ON spree_inventory_units USING btree (line_item_id);
CREATE INDEX idx_spree_inventory_units_on_original_return_item_id ON spree_inventory_units USING btree (original_return_item_id);
CREATE INDEX idx_spree_line_items_on_order_id ON spree_line_items USING btree (order_id);
CREATE INDEX idx_spree_line_items_on_tax_category_id ON spree_line_items USING btree (tax_category_id);
CREATE INDEX idx_spree_line_items_on_variant_id ON spree_line_items USING btree (variant_id);
CREATE INDEX idx_spree_log_entries_on_source_id_and_source_type ON spree_log_entries USING btree (source_id, source_type);
CREATE INDEX idx_spree_option_type_prototypes_on_option_type_id ON spree_option_type_prototypes USING btree (option_type_id);
CREATE INDEX idx_spree_option_type_prototypes_on_prototype_id ON spree_option_type_prototypes USING btree (prototype_id);
CREATE INDEX idx_spree_option_types_on_name ON spree_option_types USING btree (name);
CREATE INDEX idx_spree_option_types_on_position ON spree_option_types USING btree ("position");
CREATE INDEX idx_spree_option_value_variants_on_option_value_id ON spree_option_value_variants USING btree (option_value_id);
CREATE INDEX idx_spree_option_value_variants_on_variant_id ON spree_option_value_variants USING btree (variant_id);
CREATE INDEX idx_spree_option_values_on_name ON spree_option_values USING btree (name);
CREATE INDEX idx_spree_option_values_on_option_type_id ON spree_option_values USING btree (option_type_id);
CREATE INDEX idx_spree_option_values_on_position ON spree_option_values USING btree ("position");
CREATE INDEX idx_spree_order_promotions_on_order_id ON spree_order_promotions USING btree (order_id);
CREATE INDEX idx_spree_order_promotions_on_promotion_id ON spree_order_promotions USING btree (promotion_id);
CREATE INDEX idx_spree_order_promotions_on_promotion_id_and_order_id ON spree_order_promotions USING btree (promotion_id, order_id);
CREATE INDEX idx_spree_orders_on_approver_id ON spree_orders USING btree (approver_id);
CREATE INDEX idx_spree_orders_on_bill_address_id ON spree_orders USING btree (bill_address_id);
CREATE INDEX idx_spree_orders_on_canceler_id ON spree_orders USING btree (canceler_id);
CREATE INDEX idx_spree_orders_on_completed_at ON spree_orders USING btree (completed_at);
CREATE INDEX idx_spree_orders_on_confirmation_delivered ON spree_orders USING btree (confirmation_delivered);
CREATE INDEX idx_spree_orders_on_considered_risky ON spree_orders USING btree (considered_risky);
CREATE INDEX idx_spree_orders_on_created_by_id ON spree_orders USING btree (created_by_id);
CREATE INDEX idx_spree_orders_on_guest_token ON spree_orders USING btree (guest_token);
CREATE UNIQUE INDEX idx_spree_orders_on_number ON spree_orders USING btree (number);
CREATE INDEX idx_spree_orders_on_ship_address_id ON spree_orders USING btree (ship_address_id);
CREATE INDEX idx_spree_orders_on_store_id ON spree_orders USING btree (store_id);
CREATE INDEX idx_spree_orders_on_user_id_and_created_by_id ON spree_orders USING btree (user_id, created_by_id);
CREATE INDEX idx_spree_payment_capture_events_on_payment_id ON spree_payment_capture_events USING btree (payment_id);
CREATE INDEX idx_spree_payment_methods_on_id_and_type ON spree_payment_methods USING btree (id, type);
CREATE UNIQUE INDEX idx_spree_payments_on_number ON spree_payments USING btree (number);
CREATE INDEX idx_spree_payments_on_order_id ON spree_payments USING btree (order_id);
CREATE INDEX idx_spree_payments_on_payment_method_id ON spree_payments USING btree (payment_method_id);
CREATE INDEX idx_spree_payments_on_source_id_and_source_type ON spree_payments USING btree (source_id, source_type);
CREATE UNIQUE INDEX idx_spree_preferences_on_key ON spree_preferences USING btree (key);
CREATE INDEX idx_spree_prices_on_deleted_at ON spree_prices USING btree (deleted_at);
CREATE INDEX idx_spree_prices_on_variant_id ON spree_prices USING btree (variant_id);
CREATE INDEX idx_spree_prices_on_variant_id_and_currency ON spree_prices USING btree (variant_id, currency);
CREATE INDEX idx_spree_product_option_types_on_option_type_id ON spree_product_option_types USING btree (option_type_id);
CREATE INDEX idx_spree_product_option_types_on_position ON spree_product_option_types USING btree ("position");
CREATE INDEX idx_spree_product_option_types_on_product_id ON spree_product_option_types USING btree (product_id);
CREATE INDEX idx_spree_product_properties_on_position ON spree_product_properties USING btree ("position");
CREATE INDEX idx_spree_product_properties_on_property_id ON spree_product_properties USING btree (property_id);
CREATE INDEX idx_spree_products_on_available_on ON spree_products USING btree (available_on);
CREATE INDEX idx_spree_products_on_deleted_at ON spree_products USING btree (deleted_at);
CREATE INDEX idx_spree_products_on_discontinue_on ON spree_products USING btree (discontinue_on);
CREATE INDEX idx_spree_products_on_name ON spree_products USING btree (name);
CREATE INDEX idx_spree_products_on_shipping_category_id ON spree_products USING btree (shipping_category_id);
CREATE UNIQUE INDEX idx_spree_products_on_slug ON spree_products USING btree (slug);
CREATE INDEX idx_spree_products_on_tax_category_id ON spree_products USING btree (tax_category_id);
CREATE INDEX idx_spree_products_taxons_on_position ON spree_products_taxons USING btree ("position");
CREATE INDEX idx_spree_products_taxons_on_product_id ON spree_products_taxons USING btree (product_id);
CREATE INDEX idx_spree_products_taxons_on_taxon_id ON spree_products_taxons USING btree (taxon_id);
CREATE INDEX idx_spree_promotion_action_line_items_on_promotion_action_id ON spree_promotion_action_line_items USING btree (promotion_action_id);
CREATE INDEX idx_spree_promotion_action_line_items_on_variant_id ON spree_promotion_action_line_items USING btree (variant_id);
CREATE INDEX idx_spree_promotion_actions_on_deleted_at ON spree_promotion_actions USING btree (deleted_at);
CREATE INDEX idx_spree_promotion_actions_on_id_and_type ON spree_promotion_actions USING btree (id, type);
CREATE INDEX idx_spree_promotion_actions_on_promotion_id ON spree_promotion_actions USING btree (promotion_id);
CREATE INDEX idx_spree_promotion_rule_taxons_on_promotion_rule_id ON spree_promotion_rule_taxons USING btree (promotion_rule_id);
CREATE INDEX idx_spree_promotion_rule_taxons_on_taxon_id ON spree_promotion_rule_taxons USING btree (taxon_id);
CREATE INDEX idx_spree_promotion_rules_on_promotion_id ON spree_promotion_rules USING btree (promotion_id);
CREATE INDEX idx_spree_promotions_on_advertise ON spree_promotions USING btree (advertise);
CREATE INDEX idx_spree_promotions_on_code ON spree_promotions USING btree (code);
CREATE INDEX idx_spree_promotions_on_expires_at ON spree_promotions USING btree (expires_at);
CREATE INDEX idx_spree_promotions_on_id_and_type ON spree_promotions USING btree (id, type);
CREATE INDEX idx_spree_promotions_on_promotion_category_id ON spree_promotions USING btree (promotion_category_id);
CREATE INDEX idx_spree_promotions_on_starts_at ON spree_promotions USING btree (starts_at);
CREATE INDEX idx_spree_properties_on_name ON spree_properties USING btree (name);
CREATE INDEX idx_spree_property_prototypes_on_property_id ON spree_property_prototypes USING btree (property_id);
CREATE INDEX idx_spree_property_prototypes_on_prototype_id ON spree_property_prototypes USING btree (prototype_id);
CREATE INDEX idx_spree_prototype_taxons_on_prototype_id ON spree_prototype_taxons USING btree (prototype_id);
CREATE INDEX idx_spree_prototype_taxons_on_prototype_id_and_taxon_id ON spree_prototype_taxons USING btree (prototype_id, taxon_id);
CREATE INDEX idx_spree_prototype_taxons_on_taxon_id ON spree_prototype_taxons USING btree (taxon_id);
CREATE UNIQUE INDEX idx_spree_refund_reasons_on_lower_name ON spree_refund_reasons USING btree (lower((name)::text));
CREATE INDEX idx_spree_refunds_on_payment_id ON spree_refunds USING btree (payment_id);
CREATE INDEX idx_spree_refunds_on_reimbursement_id ON spree_refunds USING btree (reimbursement_id);
CREATE INDEX idx_spree_reimbursement_credits_on_reimbursement_id ON spree_reimbursement_credits USING btree (reimbursement_id);
CREATE UNIQUE INDEX idx_spree_reimbursement_types_on_lower_name ON spree_reimbursement_types USING btree (lower((name)::text));
CREATE INDEX idx_spree_reimbursement_types_on_type ON spree_reimbursement_types USING btree (type);
CREATE INDEX idx_spree_reimbursements_on_customer_return_id ON spree_reimbursements USING btree (customer_return_id);
CREATE UNIQUE INDEX idx_spree_reimbursements_on_number ON spree_reimbursements USING btree (number);
CREATE INDEX idx_spree_reimbursements_on_order_id ON spree_reimbursements USING btree (order_id);
CREATE UNIQUE INDEX idx_spree_return_authorization_reasons_on_lower_name ON spree_return_authorization_reasons USING btree (lower((name)::text));
CREATE UNIQUE INDEX idx_spree_return_authorizations_on_number ON spree_return_authorizations USING btree (number);
CREATE INDEX idx_spree_return_authorizations_on_order_id ON spree_return_authorizations USING btree (order_id);
CREATE INDEX idx_spree_return_authorizations_on_stock_location_id ON spree_return_authorizations USING btree (stock_location_id);
CREATE INDEX idx_spree_return_items_on_exchange_variant_id ON spree_return_items USING btree (exchange_variant_id);
CREATE INDEX idx_spree_return_items_on_inventory_unit_id ON spree_return_items USING btree (inventory_unit_id);
CREATE INDEX idx_spree_return_items_on_override_reimbursement_type_id ON spree_return_items USING btree (override_reimbursement_type_id);
CREATE INDEX idx_spree_return_items_on_preferred_reimbursement_type_id ON spree_return_items USING btree (preferred_reimbursement_type_id);
CREATE INDEX idx_spree_return_items_on_reimbursement_id ON spree_return_items USING btree (reimbursement_id);
CREATE INDEX idx_spree_return_items_on_return_authorization_id ON spree_return_items USING btree (return_authorization_id);
CREATE INDEX idx_spree_role_users_on_role_id ON spree_role_users USING btree (role_id);
CREATE INDEX idx_spree_role_users_on_user_id ON spree_role_users USING btree (user_id);
CREATE UNIQUE INDEX idx_spree_roles_on_lower_name ON spree_roles USING btree (lower((name)::text));
CREATE INDEX idx_spree_shipments_on_address_id ON spree_shipments USING btree (address_id);
CREATE UNIQUE INDEX idx_spree_shipments_on_number ON spree_shipments USING btree (number);
CREATE INDEX idx_spree_shipments_on_order_id ON spree_shipments USING btree (order_id);
CREATE INDEX idx_spree_shipments_on_stock_location_id ON spree_shipments USING btree (stock_location_id);
CREATE INDEX idx_spree_shipping_categories_on_name ON spree_shipping_categories USING btree (name);
CREATE INDEX idx_spree_shipping_method_categories_on_shipping_category_id ON spree_shipping_method_categories USING btree (shipping_category_id);
CREATE INDEX idx_spree_shipping_method_categories_on_shipping_method_id ON spree_shipping_method_categories USING btree (shipping_method_id);
CREATE INDEX idx_spree_shipping_method_zones_on_shipping_method_id ON spree_shipping_method_zones USING btree (shipping_method_id);
CREATE INDEX idx_spree_shipping_method_zones_on_zone_id ON spree_shipping_method_zones USING btree (zone_id);
CREATE INDEX idx_spree_shipping_methods_on_deleted_at ON spree_shipping_methods USING btree (deleted_at);
CREATE INDEX idx_spree_shipping_methods_on_tax_category_id ON spree_shipping_methods USING btree (tax_category_id);
CREATE INDEX idx_spree_shipping_rates_on_selected ON spree_shipping_rates USING btree (selected);
CREATE INDEX idx_spree_shipping_rates_on_shipment_id ON spree_shipping_rates USING btree (shipment_id);
CREATE INDEX idx_spree_shipping_rates_on_shipping_method_id ON spree_shipping_rates USING btree (shipping_method_id);
CREATE INDEX idx_spree_shipping_rates_on_tax_rate_id ON spree_shipping_rates USING btree (tax_rate_id);
CREATE INDEX idx_spree_state_changes_on_stateful_id_and_stateful_type ON spree_state_changes USING btree (stateful_id, stateful_type);
CREATE INDEX idx_spree_states_on_country_id ON spree_states USING btree (country_id);
CREATE INDEX idx_spree_stock_items_on_backorderable ON spree_stock_items USING btree (backorderable);
CREATE INDEX idx_spree_stock_items_on_deleted_at ON spree_stock_items USING btree (deleted_at);
CREATE INDEX idx_spree_stock_items_on_stock_location_id ON spree_stock_items USING btree (stock_location_id);
CREATE INDEX idx_spree_stock_items_on_variant_id ON spree_stock_items USING btree (variant_id);
CREATE INDEX idx_spree_stock_locations_on_active ON spree_stock_locations USING btree (active);
CREATE INDEX idx_spree_stock_locations_on_backorderable_default ON spree_stock_locations USING btree (backorderable_default);
CREATE INDEX idx_spree_stock_locations_on_country_id ON spree_stock_locations USING btree (country_id);
CREATE INDEX idx_spree_stock_locations_on_propagate_all_variants ON spree_stock_locations USING btree (propagate_all_variants);
CREATE INDEX idx_spree_stock_locations_on_state_id ON spree_stock_locations USING btree (state_id);
CREATE INDEX idx_spree_stock_movements_on_stock_item_id ON spree_stock_movements USING btree (stock_item_id);
CREATE INDEX idx_spree_stock_transfers_on_destination_location_id ON spree_stock_transfers USING btree (destination_location_id);
CREATE UNIQUE INDEX idx_spree_stock_transfers_on_number ON spree_stock_transfers USING btree (number);
CREATE INDEX idx_spree_stock_transfers_on_source_location_id ON spree_stock_transfers USING btree (source_location_id);
CREATE INDEX idx_spree_store_credit_events_on_store_credit_id ON spree_store_credit_events USING btree (store_credit_id);
CREATE INDEX idx_spree_store_credit_types_on_priority ON spree_store_credit_types USING btree (priority);
CREATE INDEX idx_spree_store_credits_on_deleted_at ON spree_store_credits USING btree (deleted_at);
CREATE INDEX idx_spree_store_credits_on_type_id ON spree_store_credits USING btree (type_id);
CREATE INDEX idx_spree_store_credits_on_user_id ON spree_store_credits USING btree (user_id);
CREATE INDEX idx_spree_stores_on_default ON spree_stores USING btree ("default");
CREATE UNIQUE INDEX idx_spree_stores_on_lower_code ON spree_stores USING btree (lower((code)::text));
CREATE INDEX idx_spree_stores_on_url ON spree_stores USING btree (url);
CREATE INDEX idx_spree_taggings_on_context ON spree_taggings USING btree (context);
CREATE INDEX idx_spree_taggings_on_tag_id ON spree_taggings USING btree (tag_id);
CREATE INDEX idx_spree_taggings_on_taggable_id ON spree_taggings USING btree (taggable_id);
CREATE INDEX idx_spree_taggings_on_taggable_type ON spree_taggings USING btree (taggable_type);
CREATE INDEX idx_spree_taggings_on_tagger_id ON spree_taggings USING btree (tagger_id);
CREATE INDEX idx_spree_taggings_on_tagger_id_and_tagger_type ON spree_taggings USING btree (tagger_id, tagger_type);
CREATE UNIQUE INDEX idx_spree_tags_on_name ON spree_tags USING btree (name);
CREATE INDEX idx_spree_tax_categories_on_deleted_at ON spree_tax_categories USING btree (deleted_at);
CREATE INDEX idx_spree_tax_categories_on_is_default ON spree_tax_categories USING btree (is_default);
CREATE INDEX idx_spree_tax_rates_on_deleted_at ON spree_tax_rates USING btree (deleted_at);
CREATE INDEX idx_spree_tax_rates_on_included_in_price ON spree_tax_rates USING btree (included_in_price);
CREATE INDEX idx_spree_tax_rates_on_show_rate_in_label ON spree_tax_rates USING btree (show_rate_in_label);
CREATE INDEX idx_spree_tax_rates_on_tax_category_id ON spree_tax_rates USING btree (tax_category_id);
CREATE INDEX idx_spree_tax_rates_on_zone_id ON spree_tax_rates USING btree (zone_id);
CREATE INDEX idx_spree_taxonomies_on_position ON spree_taxonomies USING btree ("position");
CREATE INDEX idx_spree_taxons_on_lft ON spree_taxons USING btree (lft);
CREATE INDEX idx_spree_taxons_on_name ON spree_taxons USING btree (name);
CREATE INDEX idx_spree_taxons_on_position ON spree_taxons USING btree ("position");
CREATE INDEX idx_spree_taxons_on_rgt ON spree_taxons USING btree (rgt);
CREATE INDEX idx_spree_trackers_on_active ON spree_trackers USING btree (active);
CREATE INDEX idx_spree_users_on_bill_address_id ON spree_users USING btree (bill_address_id);
CREATE INDEX idx_spree_users_on_deleted_at ON spree_users USING btree (deleted_at);
CREATE INDEX idx_spree_users_on_ship_address_id ON spree_users USING btree (ship_address_id);
CREATE INDEX idx_spree_users_on_spree_api_key ON spree_users USING btree (spree_api_key);
CREATE INDEX idx_spree_variants_on_deleted_at ON spree_variants USING btree (deleted_at);
CREATE INDEX idx_spree_variants_on_discontinue_on ON spree_variants USING btree (discontinue_on);
CREATE INDEX idx_spree_variants_on_is_master ON spree_variants USING btree (is_master);
CREATE INDEX idx_spree_variants_on_position ON spree_variants USING btree ("position");
CREATE INDEX idx_spree_variants_on_product_id ON spree_variants USING btree (product_id);
CREATE INDEX idx_spree_variants_on_sku ON spree_variants USING btree (sku);
CREATE INDEX idx_spree_variants_on_tax_category_id ON spree_variants USING btree (tax_category_id);
CREATE INDEX idx_spree_variants_on_track_inventory ON spree_variants USING btree (track_inventory);
CREATE INDEX idx_spree_zone_members_on_zone_id ON spree_zone_members USING btree (zone_id);
CREATE INDEX idx_spree_zone_members_on_zoneable_id_and_zoneable_type ON spree_zone_members USING btree (zoneable_id, zoneable_type);
CREATE INDEX idx_spree_zones_on_default_tax ON spree_zones USING btree (default_tax);
CREATE INDEX idx_spree_zones_on_kind ON spree_zones USING btree (kind);
CREATE INDEX idx_spree_stock_movements_on_originator_id_and_originator_type ON spree_stock_movements USING btree (originator_id, originator_type);
CREATE INDEX idx_spree_taxons_on_parent_id ON spree_taxons USING btree (parent_id);
CREATE INDEX idx_spree_taxons_on_permalink ON spree_taxons USING btree (permalink);
CREATE INDEX idx_spree_taxons_on_taxonomy_id ON spree_taxons USING btree (taxonomy_id);
CREATE UNIQUE INDEX idx_spree_option_type_prototypes_prototype_id_option_type_id ON spree_option_type_prototypes USING btree (prototype_id, option_type_id);
CREATE UNIQUE INDEX idx_spree_shipping_rates_join_index ON spree_shipping_rates USING btree (shipment_id, shipping_method_id);
CREATE INDEX idx_spree_store_credit_events_originator ON spree_store_credit_events USING btree (originator_id, originator_type);
CREATE INDEX idx_spree_store_credits_originator ON spree_store_credits USING btree (originator_id, originator_type);
CREATE UNIQUE INDEX idx_spree_taggings_idx ON spree_taggings USING btree (tag_id, taggable_id, taggable_type, context, tagger_id, tagger_type);
CREATE INDEX idx_spree_taggings_idy ON spree_taggings USING btree (taggable_id, taggable_type, tagger_id, context);
CREATE INDEX idx_spree_stock_item_by_loc_and_var_id ON spree_stock_items USING btree (stock_location_id, variant_id);
CREATE UNIQUE INDEX idx_spree_shipping_method_and_categories ON spree_shipping_method_categories USING btree (shipping_category_id, shipping_method_id);
