CREATE TABLE friendly_id_slugs (
    id BIGSERIAL PRIMARY KEY,
    slug varchar(255) NOT NULL,
    sluggable_id BIGINT NOT NULL,
    sluggable_type varchar(50) NOT NULL,
    scope varchar NOT NULL,
    created_at timestamp without time zone DEFAULT NOW(),
    deleted_at timestamp without time zone
);

CREATE TABLE spree_addresses (
    id BIGSERIAL PRIMARY KEY,
    firstname varchar(32) NOT NULL,
    lastname varchar(32) NOT NULL,
    address1 varchar(255) NOT NULL,
    address2 varchar(255),
    city varchar(64) NOT NULL,
    zipcode varchar(16) NOT NULL,
    phone varchar(32) NOT NULL,
    state_name varchar(64) NOT NULL,
    alternative_phone varchar(32),
    company varchar(255),
    state_id BIGINT NOT NULL,
    country_id BIGINT NOT NULL,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_adjustments (
    id BIGSERIAL PRIMARY KEY,
    source_type varchar(255) NOT NULL,
    source_id BIGINT NOT NULL,
    adjustable_type varchar(255) NOT NULL,
    adjustable_id BIGINT NOT NULL,
    amount numeric(10,2) NOT NULL,
    label varchar(255) NOT NULL,
    mandatory boolean,
    eligible boolean DEFAULT true,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    state varchar,
    order_id BIGINT NOT NULL,
    included boolean DEFAULT false
);

CREATE TABLE spree_assets (
    id BIGSERIAL PRIMARY KEY,
    viewable_type varchar,
    viewable_id BIGINT,
    attachment_width integer,
    attachment_height integer,
    attachment_file_size integer,
    "position" integer,
    attachment_content_type varchar,
    attachment_file_name varchar,
    type varchar(75),
    attachment_updated_at timestamp without time zone,
    alt text,
    created_at timestamp without time zone DEFAULT NOW(),
    updated_at timestamp without time zone
);

CREATE TABLE spree_calculators (
    id BIGSERIAL PRIMARY KEY,
    type varchar,
    calculable_type varchar,
    calculable_id BIGINT,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    preferences text,
    deleted_at timestamp without time zone
);

CREATE TABLE spree_countries (
    id BIGSERIAL PRIMARY KEY,
    name varchar(255) NOT NULL,
    iso_name varchar(255) NOT NULL,
    numcode integer NOT NULL,
    iso varchar(2) NOT NULL,
    iso3 varchar(3) NOT NULL,
    states_required boolean NOT NULL DEFAULT false,
    zipcode_required boolean NOT NULL DEFAULT true,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_credit_cards (
    id BIGSERIAL PRIMARY KEY,
    month varchar,
    year varchar,
    cc_type varchar,
    last_digits varchar,
    address_id BIGINT,
    gateway_customer_profile_id varchar,
    gateway_payment_profile_id varchar,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    name varchar,
    user_id BIGINT,
    payment_method_id BIGINT,
    "default" boolean DEFAULT false NOT NULL
);

CREATE TABLE spree_customer_returns (
    id BIGSERIAL PRIMARY KEY,
    number varchar,
    stock_location_id BIGINT,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_gateways (
    id BIGSERIAL PRIMARY KEY,
    type varchar,
    name varchar,
    description text,
    active boolean DEFAULT true,
    environment varchar DEFAULT 'development',
    server varchar DEFAULT 'test',
    test_mode boolean DEFAULT true,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    preferences text
);

CREATE TABLE spree_inventory_units (
    id BIGSERIAL PRIMARY KEY,
    state varchar,
    variant_id BIGINT,
    order_id BIGINT,
    shipment_id BIGINT,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
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
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    currency varchar,
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
    source_type varchar,
    source_id BIGINT,
    details text,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_option_type_prototypes (
    id BIGSERIAL PRIMARY KEY,
    prototype_id BIGINT,
    option_type_id BIGINT
);

CREATE TABLE spree_option_types (
    id BIGSERIAL PRIMARY KEY,
    name varchar(100),
    presentation varchar(100),
    "position" integer DEFAULT 0 NOT NULL,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
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
    name varchar,
    presentation varchar,
    option_type_id BIGINT,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_order_promotions (
    id BIGSERIAL PRIMARY KEY,
    order_id BIGINT,
    promotion_id BIGINT
);

CREATE TABLE spree_orders (
    id BIGSERIAL PRIMARY KEY,
    number varchar(32),
    item_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    total numeric(10,2) DEFAULT 0.0 NOT NULL,
    state varchar,
    adjustment_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    user_id BIGINT,
    completed_at timestamp without time zone,
    bill_address_id BIGINT,
    ship_address_id BIGINT,
    payment_total numeric(10,2) DEFAULT 0.0,
    shipment_state varchar,
    payment_state varchar,
    email varchar,
    special_instructions text,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    currency varchar,
    last_ip_address varchar,
    created_by_id BIGINT,
    shipment_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    additional_tax_total numeric(10,2) DEFAULT 0.0,
    promo_total numeric(10,2) DEFAULT 0.0,
    channel varchar DEFAULT 'spree',
    included_tax_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    item_count integer DEFAULT 0,
    approver_id BIGINT,
    approved_at timestamp without time zone,
    confirmation_delivered boolean DEFAULT false,
    considered_risky boolean DEFAULT false,
    guest_token varchar,
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
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_payment_methods (
    id BIGSERIAL PRIMARY KEY,
    type varchar,
    name varchar,
    description text,
    active boolean DEFAULT true,
    deleted_at timestamp without time zone,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    display_on varchar DEFAULT 'both',
    auto_capture boolean,
    preferences text,
    "position" integer DEFAULT 0
);

CREATE TABLE spree_payments (
    id BIGSERIAL PRIMARY KEY,
    amount numeric(10,2) DEFAULT 0.0 NOT NULL,
    order_id BIGINT,
    source_type varchar,
    source_id BIGINT,
    payment_method_id BIGINT,
    state varchar,
    response_code varchar,
    avs_response varchar,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    number varchar,
    cvv_response_code varchar,
    cvv_response_message varchar
);

CREATE TABLE spree_preferences (
    id BIGSERIAL PRIMARY KEY,
    value text,
    key varchar,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_prices (
    id BIGSERIAL PRIMARY KEY,
    variant_id BIGINT NOT NULL,
    amount numeric(10,2),
    currency varchar,
    deleted_at timestamp without time zone
);

CREATE TABLE spree_product_option_types (
    id BIGSERIAL PRIMARY KEY,
    "position" integer,
    product_id BIGINT,
    option_type_id BIGINT,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_product_promotion_rules (
    id BIGSERIAL PRIMARY KEY,
    product_id BIGINT,
    promotion_rule_id BIGINT
);

CREATE TABLE spree_product_properties (
    id BIGSERIAL PRIMARY KEY,
    value varchar,
    product_id BIGINT,
    property_id BIGINT,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    "position" integer DEFAULT 0
);

CREATE TABLE spree_products (
    id BIGSERIAL PRIMARY KEY,
    name varchar DEFAULT '' NOT NULL,
    description text,
    available_on timestamp without time zone,
    deleted_at timestamp without time zone,
    slug varchar,
    meta_description text,
    meta_keywords varchar,
    tax_category_id BIGINT,
    shipping_category_id BIGINT,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    promotionable boolean DEFAULT true,
    meta_title varchar,
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
    type varchar,
    deleted_at timestamp without time zone
);

CREATE TABLE spree_promotion_categories (
    id BIGSERIAL PRIMARY KEY,
    name varchar,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    code varchar
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
    type varchar,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    code varchar,
    preferences text
);

CREATE TABLE spree_promotions (
    id BIGSERIAL PRIMARY KEY,
    description TEXT NOT NULL,
    expires_at timestamp without time zone,
    starts_at timestamp without time zone,
    name varchar,
    type varchar,
    usage_limit integer,
    match_policy varchar DEFAULT 'all',
    code varchar,
    advertise boolean DEFAULT false,
    path varchar,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    promotion_category_id BIGINT
);

CREATE TABLE spree_properties (
    id BIGSERIAL PRIMARY KEY,
    name varchar,
    presentation varchar NOT NULL,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
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
    name varchar,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_refund_reasons (
    id BIGSERIAL PRIMARY KEY,
    name varchar,
    active boolean DEFAULT true,
    mutable boolean DEFAULT true,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_refunds (
    id BIGSERIAL PRIMARY KEY,
    payment_id BIGINT,
    amount numeric(10,2) DEFAULT 0.0 NOT NULL,
    transaction_id varchar,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    refund_reason_id BIGINT,
    reimbursement_id BIGINT
);

CREATE TABLE spree_reimbursement_credits (
    id BIGSERIAL PRIMARY KEY,
    amount numeric(10,2) DEFAULT 0.0 NOT NULL,
    reimbursement_id BIGINT,
    creditable_id BIGINT,
    creditable_type varchar
);

CREATE TABLE spree_reimbursement_types (
    id BIGSERIAL PRIMARY KEY,
    name varchar,
    active boolean DEFAULT true,
    mutable boolean DEFAULT true,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    type varchar
);

CREATE TABLE spree_reimbursements (
    id BIGSERIAL PRIMARY KEY,
    number varchar,
    reimbursement_status varchar,
    customer_return_id BIGINT,
    order_id BIGINT,
    total numeric(10,2),
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_return_authorization_reasons (
    id BIGSERIAL PRIMARY KEY,
    name varchar,
    active boolean DEFAULT true,
    mutable boolean DEFAULT true,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_return_authorizations (
    id BIGSERIAL PRIMARY KEY,
    number varchar,
    state varchar,
    order_id BIGINT,
    memo text,
    created_at timestamp without time zone DEFAULT NOW(),
    updated_at timestamp without time zone,
    stock_location_id BIGINT,
    return_authorization_reason_id BIGINT
);

CREATE TABLE spree_return_items (
    id BIGSERIAL PRIMARY KEY,
    return_authorization_id BIGINT,
    inventory_unit_id BIGINT,
    exchange_variant_id BIGINT,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    pre_tax_amount numeric(12,4) DEFAULT 0.0 NOT NULL,
    included_tax_total numeric(12,4) DEFAULT 0.0 NOT NULL,
    additional_tax_total numeric(12,4) DEFAULT 0.0 NOT NULL,
    reception_status varchar,
    acceptance_status varchar,
    customer_return_id BIGINT,
    reimbursement_id BIGINT,
    acceptance_status_errors text,
    preferred_reimbursement_type_id BIGINT,
    override_reimbursement_type_id BIGINT,
    resellable boolean DEFAULT true NOT NULL
);

CREATE TABLE spree_role_users (
    id BIGSERIAL PRIMARY KEY,
    role_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    nbf DATE NOT NULL DEFAULT current_date,
    "exp" DATE NOT NULL,
    created_at timestamp without time zone DEFAULT NOW(),
    updated_at timestamp without time zone
);

CREATE TABLE spree_roles (
    id BIGSERIAL PRIMARY KEY,
    name varchar NOT NULL
);

CREATE TABLE spree_shipments (
    id BIGSERIAL PRIMARY KEY,
    tracking varchar,
    number varchar,
    cost numeric(10,2) DEFAULT 0.0,
    shipped_at timestamp without time zone,
    order_id BIGINT,
    address_id BIGINT,
    state varchar,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
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
    name varchar,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_shipping_method_categories (
    id BIGSERIAL PRIMARY KEY,
    shipping_method_id BIGINT NOT NULL,
    shipping_category_id BIGINT NOT NULL,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_shipping_method_zones (
    id BIGSERIAL PRIMARY KEY,
    shipping_method_id BIGINT,
    zone_id BIGINT
);

CREATE TABLE spree_shipping_methods (
    id BIGSERIAL PRIMARY KEY,
    name varchar,
    display_on varchar,
    deleted_at timestamp without time zone,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    tracking_url varchar,
    admin_name varchar,
    tax_category_id BIGINT,
    code varchar
);

CREATE TABLE spree_shipping_rates (
    id BIGSERIAL PRIMARY KEY,
    shipment_id BIGINT,
    shipping_method_id BIGINT,
    selected boolean DEFAULT false,
    cost numeric(8,2) DEFAULT 0,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    tax_rate_id BIGINT
);

CREATE TABLE spree_state_changes (
    id BIGSERIAL PRIMARY KEY,
    name varchar,
    previous_state varchar,
    stateful_id BIGINT,
    user_id BIGINT,
    stateful_type varchar,
    next_state varchar,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_states (
    id BIGSERIAL PRIMARY KEY,
    name varchar(255) NOT NULL,
    abbr varchar(32) NOT NULL,
    country_id BIGINT NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_stock_items (
    id BIGSERIAL PRIMARY KEY,
    stock_location_id BIGINT,
    variant_id BIGINT,
    count_on_hand integer DEFAULT 0 NOT NULL,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    backorderable boolean DEFAULT false,
    deleted_at timestamp without time zone
);

CREATE TABLE spree_stock_locations (
    id BIGSERIAL PRIMARY KEY,
    name varchar,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    "default" boolean DEFAULT false NOT NULL,
    address1 varchar,
    address2 varchar,
    city varchar,
    state_id BIGINT,
    state_name varchar,
    country_id BIGINT,
    zipcode varchar,
    phone varchar,
    active boolean DEFAULT true,
    backorderable_default boolean DEFAULT false,
    propagate_all_variants boolean DEFAULT true,
    admin_name varchar
);

CREATE TABLE spree_stock_movements (
    id BIGSERIAL PRIMARY KEY,
    stock_item_id BIGINT,
    quantity integer DEFAULT 0,
    action varchar,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    originator_type varchar,
    originator_id BIGINT
);

CREATE TABLE spree_stock_transfers (
    id BIGSERIAL PRIMARY KEY,
    type varchar,
    reference varchar,
    source_location_id BIGINT,
    destination_location_id BIGINT,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    number varchar
);

CREATE TABLE spree_store_credit_categories (
    id BIGSERIAL PRIMARY KEY,
    name varchar,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_store_credit_events (
    id BIGSERIAL PRIMARY KEY,
    store_credit_id BIGINT NOT NULL,
    action varchar NOT NULL,
    amount numeric(8,2),
    authorization_code varchar NOT NULL,
    user_total_amount numeric(8,2) DEFAULT 0.0 NOT NULL,
    originator_id BIGINT,
    originator_type varchar,
    deleted_at timestamp without time zone,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_store_credit_types (
    id BIGSERIAL PRIMARY KEY,
    name varchar,
    priority integer,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
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
    currency varchar,
    amount_authorized numeric(8,2) DEFAULT 0.0 NOT NULL,
    originator_id BIGINT,
    originator_type varchar,
    type_id BIGINT,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_stores (
    id BIGSERIAL PRIMARY KEY,
    name varchar,
    url varchar,
    meta_description text,
    meta_keywords text,
    seo_title varchar,
    mail_from_address varchar,
    default_currency varchar,
    code varchar,
    "default" boolean DEFAULT false NOT NULL,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_taggings (
    id BIGSERIAL PRIMARY KEY,
    tag_id BIGINT,
    taggable_type varchar,
    taggable_id BIGINT,
    tagger_type varchar,
    tagger_id BIGINT,
    context varchar(128),
    created_at timestamp without time zone
);

CREATE TABLE spree_tags (
    id BIGSERIAL PRIMARY KEY,
    name varchar,
    taggings_count integer DEFAULT 0
);

CREATE TABLE spree_tax_categories (
    id BIGSERIAL PRIMARY KEY,
    name varchar,
    description TEXT NOT NULL,
    is_default boolean DEFAULT false,
    deleted_at timestamp without time zone,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    tax_code varchar
);

CREATE TABLE spree_tax_rates (
    id BIGSERIAL PRIMARY KEY,
    amount numeric(8,5),
    zone_id BIGINT,
    tax_category_id BIGINT,
    included_in_price boolean DEFAULT false,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    name varchar,
    show_rate_in_label boolean DEFAULT true,
    deleted_at timestamp without time zone
);

CREATE TABLE spree_taxonomies (
    id BIGSERIAL PRIMARY KEY,
    name varchar NOT NULL,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    "position" integer DEFAULT 0
);

CREATE TABLE spree_taxons (
    id BIGSERIAL PRIMARY KEY,
    parent_id BIGINT,
    "position" integer DEFAULT 0,
    name varchar NOT NULL,
    permalink varchar,
    taxonomy_id BIGINT,
    lft integer,
    rgt integer,
    icon_file_name varchar,
    icon_content_type varchar,
    icon_file_size integer,
    icon_updated_at timestamp without time zone,
    description text,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    meta_title varchar,
    meta_description TEXT NOT NULL,
    meta_keywords varchar,
    depth integer
);

CREATE TABLE spree_trackers (
    id BIGSERIAL PRIMARY KEY,
    analytics_id varchar,
    active boolean DEFAULT true,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    engine integer DEFAULT 0 NOT NULL
);

CREATE TABLE spree_users (
    id BIGSERIAL PRIMARY KEY,
    encrypted_password varchar NOT NULL,
    password_salt varchar(128),
    email varchar(255) NOT NULL,
    remember_token varchar,
    persistence_token varchar,
    reset_password_token varchar,
    perishable_token varchar,
    sign_in_count integer DEFAULT 0 NOT NULL,
    failed_attempts integer DEFAULT 0 NOT NULL,
    last_request_at timestamp without time zone,
    current_sign_in_at timestamp without time zone,
    last_sign_in_at timestamp without time zone,
    current_sign_in_ip varchar(46),
    last_sign_in_ip varchar(46),
    login varchar,
    ship_address_id BIGINT,
    bill_address_id BIGINT,
    authentication_token varchar,
    unlock_token varchar,
    locked_at timestamp without time zone,
    reset_password_sent_at timestamp without time zone,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    spree_api_key varchar(48),
    remember_created_at timestamp without time zone DEFAULT NOW(),
    deleted_at timestamp without time zone,
    confirmation_token varchar,
    confirmed_at timestamp without time zone,
    confirmation_sent_at timestamp without time zone
);

CREATE TABLE spree_variants (
    id BIGSERIAL PRIMARY KEY,
    sku varchar DEFAULT '' NOT NULL,
    weight numeric(8,2) DEFAULT 0.0,
    height numeric(8,2),
    width numeric(8,2),
    depth numeric(8,2),
    deleted_at timestamp without time zone,
    is_master boolean DEFAULT false,
    product_id BIGINT,
    cost_price numeric(10,2),
    "position" integer,
    cost_currency varchar,
    track_inventory boolean DEFAULT true,
    tax_category_id BIGINT,
    updated_at timestamp without time zone NOT NULL,
    discontinue_on timestamp without time zone,
    created_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_zone_members (
    id BIGSERIAL PRIMARY KEY,
    zone_id BIGINT NOT NULL,
    zoneable_type varchar(16) NOT NULL,
    zoneable_id BIGINT NOT NULL,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE TABLE spree_zones (
    id BIGSERIAL PRIMARY KEY,
    name varchar(255) NOT NULL,
    description TEXT NOT NULL,
    kind varchar(16) NOT NULL,
    default_tax boolean DEFAULT false NOT NULL,
    zone_members_count integer DEFAULT 0 NOT NULL,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


CREATE UNIQUE INDEX idx_spree_users_on_email ON spree_users(email);
CREATE INDEX idx_spree_addresses_on_firstname ON spree_addresses(firstname);
CREATE INDEX idx_spree_addresses_on_lastname ON spree_addresses(lastname);
CREATE INDEX idx_spree_assets_on_viewable_id ON spree_assets(viewable_id);
CREATE INDEX idx_spree_assets_on_viewable_type_and_type ON spree_assets(viewable_type, type);

CREATE INDEX idx_friendly_id_slugs_on_deleted_at ON friendly_id_slugs(deleted_at);
CREATE INDEX idx_friendly_id_slugs_on_slug_and_sluggable_type ON friendly_id_slugs(slug, sluggable_type);
CREATE UNIQUE INDEX idx_friendly_id_slugs_on_slug_and_sluggable_type_and_scope ON friendly_id_slugs(slug, sluggable_type, scope);
CREATE INDEX idx_friendly_id_slugs_on_sluggable_id ON friendly_id_slugs(sluggable_id);
CREATE INDEX idx_friendly_id_slugs_on_sluggable_type ON friendly_id_slugs(sluggable_type);

CREATE INDEX idx_spree_inventory_units_on_order_id ON spree_inventory_units(order_id);
CREATE INDEX idx_spree_inventory_units_on_shipment_id ON spree_inventory_units(shipment_id);
CREATE INDEX idx_spree_inventory_units_on_variant_id ON spree_inventory_units(variant_id);
CREATE UNIQUE INDEX idx_spree_option_values_variants_on_variant_id_and_option_value_id ON spree_option_value_variants(variant_id, option_value_id);
CREATE INDEX idx_spree_product_properties_on_product_id ON spree_product_properties(product_id);
CREATE INDEX idx_spree_products_promotion_rules_on_product_id ON spree_product_promotion_rules(product_id);
CREATE INDEX idx_spree_products_promotion_rules_on_promotion_rule_and_product ON spree_product_promotion_rules(promotion_rule_id, product_id);
CREATE INDEX idx_spree_promotion_rules_on_product_group_id ON spree_promotion_rules(product_group_id);
CREATE INDEX idx_spree_promotion_rules_on_user_id ON spree_promotion_rules(user_id);
CREATE INDEX idx_spree_promotion_rules_users_on_promotion_rule_id ON spree_promotion_rule_users(promotion_rule_id);
CREATE INDEX idx_spree_promotion_rules_users_on_user_id_and_promotion_rule_id ON spree_promotion_rule_users(user_id, promotion_rule_id);
CREATE UNIQUE INDEX idx_spree_property_prototypes_on_prototype_id_and_property_id ON spree_property_prototypes(prototype_id, property_id);
CREATE INDEX idx_spree_refunds_on_refund_reason_id ON spree_refunds(refund_reason_id);
CREATE INDEX idx_spree_reimbursement_credits_on_creditable_id_and_type ON spree_reimbursement_credits(creditable_id, creditable_type);
CREATE INDEX idx_spree_return_authorizations_on_return_authorization_reason_id ON spree_return_authorizations(return_authorization_reason_id);
CREATE INDEX idx_spree_return_items_on_customer_return_id ON spree_return_items(customer_return_id);
CREATE INDEX idx_spree_addresses_on_country_id ON spree_addresses(country_id);
CREATE INDEX idx_spree_addresses_on_state_id ON spree_addresses(state_id);
CREATE INDEX idx_spree_adjustments_on_adjustable_id_and_adjustable_type ON spree_adjustments(adjustable_id, adjustable_type);
CREATE INDEX idx_spree_adjustments_on_eligible ON spree_adjustments(eligible);
CREATE INDEX idx_spree_adjustments_on_order_id ON spree_adjustments(order_id);
CREATE INDEX idx_spree_adjustments_on_source_id_and_source_type ON spree_adjustments(source_id, source_type);
CREATE INDEX idx_spree_assets_on_position ON spree_assets("position");
CREATE INDEX idx_spree_calculators_on_calculable_id_and_calculable_type ON spree_calculators(calculable_id, calculable_type);
CREATE INDEX idx_spree_calculators_on_deleted_at ON spree_calculators(deleted_at);
CREATE INDEX idx_spree_calculators_on_id_and_type ON spree_calculators(id, type);
CREATE UNIQUE INDEX idx_spree_countries_on_lower_iso_name ON spree_countries(lower((iso_name)::text));
CREATE UNIQUE INDEX idx_spree_countries_on_lower_name ON spree_countries(lower((name)::text));
CREATE INDEX idx_spree_credit_cards_on_address_id ON spree_credit_cards(address_id);
CREATE INDEX idx_spree_credit_cards_on_payment_method_id ON spree_credit_cards(payment_method_id);
CREATE INDEX idx_spree_credit_cards_on_user_id ON spree_credit_cards(user_id);
CREATE UNIQUE INDEX idx_spree_customer_returns_on_number ON spree_customer_returns(number);
CREATE INDEX idx_spree_customer_returns_on_stock_location_id ON spree_customer_returns(stock_location_id);
CREATE INDEX idx_spree_gateways_on_active ON spree_gateways(active);
CREATE INDEX idx_spree_gateways_on_test_mode ON spree_gateways(test_mode);
CREATE INDEX idx_spree_inventory_units_on_line_item_id ON spree_inventory_units(line_item_id);
CREATE INDEX idx_spree_inventory_units_on_original_return_item_id ON spree_inventory_units(original_return_item_id);
CREATE INDEX idx_spree_line_items_on_order_id ON spree_line_items(order_id);
CREATE INDEX idx_spree_line_items_on_tax_category_id ON spree_line_items(tax_category_id);
CREATE INDEX idx_spree_line_items_on_variant_id ON spree_line_items(variant_id);
CREATE INDEX idx_spree_log_entries_on_source_id_and_source_type ON spree_log_entries(source_id, source_type);
CREATE INDEX idx_spree_option_type_prototypes_on_option_type_id ON spree_option_type_prototypes(option_type_id);
CREATE INDEX idx_spree_option_type_prototypes_on_prototype_id ON spree_option_type_prototypes(prototype_id);
CREATE INDEX idx_spree_option_types_on_name ON spree_option_types(name);
CREATE INDEX idx_spree_option_types_on_position ON spree_option_types("position");
CREATE INDEX idx_spree_option_value_variants_on_option_value_id ON spree_option_value_variants(option_value_id);
CREATE INDEX idx_spree_option_value_variants_on_variant_id ON spree_option_value_variants(variant_id);
CREATE INDEX idx_spree_option_values_on_name ON spree_option_values(name);
CREATE INDEX idx_spree_option_values_on_option_type_id ON spree_option_values(option_type_id);
CREATE INDEX idx_spree_option_values_on_position ON spree_option_values("position");
CREATE INDEX idx_spree_order_promotions_on_order_id ON spree_order_promotions(order_id);
CREATE INDEX idx_spree_order_promotions_on_promotion_id ON spree_order_promotions(promotion_id);
CREATE INDEX idx_spree_order_promotions_on_promotion_id_and_order_id ON spree_order_promotions(promotion_id, order_id);
CREATE INDEX idx_spree_orders_on_approver_id ON spree_orders(approver_id);
CREATE INDEX idx_spree_orders_on_bill_address_id ON spree_orders(bill_address_id);
CREATE INDEX idx_spree_orders_on_canceler_id ON spree_orders(canceler_id);
CREATE INDEX idx_spree_orders_on_completed_at ON spree_orders(completed_at);
CREATE INDEX idx_spree_orders_on_confirmation_delivered ON spree_orders(confirmation_delivered);
CREATE INDEX idx_spree_orders_on_considered_risky ON spree_orders(considered_risky);
CREATE INDEX idx_spree_orders_on_created_by_id ON spree_orders(created_by_id);
CREATE INDEX idx_spree_orders_on_guest_token ON spree_orders(guest_token);
CREATE UNIQUE INDEX idx_spree_orders_on_number ON spree_orders(number);
CREATE INDEX idx_spree_orders_on_ship_address_id ON spree_orders(ship_address_id);
CREATE INDEX idx_spree_orders_on_store_id ON spree_orders(store_id);
CREATE INDEX idx_spree_orders_on_user_id_and_created_by_id ON spree_orders(user_id, created_by_id);
CREATE INDEX idx_spree_payment_capture_events_on_payment_id ON spree_payment_capture_events(payment_id);
CREATE INDEX idx_spree_payment_methods_on_id_and_type ON spree_payment_methods(id, type);
CREATE UNIQUE INDEX idx_spree_payments_on_number ON spree_payments(number);
CREATE INDEX idx_spree_payments_on_order_id ON spree_payments(order_id);
CREATE INDEX idx_spree_payments_on_payment_method_id ON spree_payments(payment_method_id);
CREATE INDEX idx_spree_payments_on_source_id_and_source_type ON spree_payments(source_id, source_type);
CREATE UNIQUE INDEX idx_spree_preferences_on_key ON spree_preferences(key);
CREATE INDEX idx_spree_prices_on_deleted_at ON spree_prices(deleted_at);
CREATE INDEX idx_spree_prices_on_variant_id ON spree_prices(variant_id);
CREATE INDEX idx_spree_prices_on_variant_id_and_currency ON spree_prices(variant_id, currency);
CREATE INDEX idx_spree_product_option_types_on_option_type_id ON spree_product_option_types(option_type_id);
CREATE INDEX idx_spree_product_option_types_on_position ON spree_product_option_types("position");
CREATE INDEX idx_spree_product_option_types_on_product_id ON spree_product_option_types(product_id);
CREATE INDEX idx_spree_product_properties_on_position ON spree_product_properties("position");
CREATE INDEX idx_spree_product_properties_on_property_id ON spree_product_properties(property_id);
CREATE INDEX idx_spree_products_on_available_on ON spree_products(available_on);
CREATE INDEX idx_spree_products_on_deleted_at ON spree_products(deleted_at);
CREATE INDEX idx_spree_products_on_discontinue_on ON spree_products(discontinue_on);
CREATE INDEX idx_spree_products_on_name ON spree_products(name);
CREATE INDEX idx_spree_products_on_shipping_category_id ON spree_products(shipping_category_id);
CREATE UNIQUE INDEX idx_spree_products_on_slug ON spree_products(slug);
CREATE INDEX idx_spree_products_on_tax_category_id ON spree_products(tax_category_id);
CREATE INDEX idx_spree_products_taxons_on_position ON spree_products_taxons("position");
CREATE INDEX idx_spree_products_taxons_on_product_id ON spree_products_taxons(product_id);
CREATE INDEX idx_spree_products_taxons_on_taxon_id ON spree_products_taxons(taxon_id);
CREATE INDEX idx_spree_promotion_action_line_items_on_promotion_action_id ON spree_promotion_action_line_items(promotion_action_id);
CREATE INDEX idx_spree_promotion_action_line_items_on_variant_id ON spree_promotion_action_line_items(variant_id);
CREATE INDEX idx_spree_promotion_actions_on_deleted_at ON spree_promotion_actions(deleted_at);
CREATE INDEX idx_spree_promotion_actions_on_id_and_type ON spree_promotion_actions(id, type);
CREATE INDEX idx_spree_promotion_actions_on_promotion_id ON spree_promotion_actions(promotion_id);
CREATE INDEX idx_spree_promotion_rule_taxons_on_promotion_rule_id ON spree_promotion_rule_taxons(promotion_rule_id);
CREATE INDEX idx_spree_promotion_rule_taxons_on_taxon_id ON spree_promotion_rule_taxons(taxon_id);
CREATE INDEX idx_spree_promotion_rules_on_promotion_id ON spree_promotion_rules(promotion_id);
CREATE INDEX idx_spree_promotions_on_advertise ON spree_promotions(advertise);
CREATE INDEX idx_spree_promotions_on_code ON spree_promotions(code);
CREATE INDEX idx_spree_promotions_on_expires_at ON spree_promotions(expires_at);
CREATE INDEX idx_spree_promotions_on_id_and_type ON spree_promotions(id, type);
CREATE INDEX idx_spree_promotions_on_promotion_category_id ON spree_promotions(promotion_category_id);
CREATE INDEX idx_spree_promotions_on_starts_at ON spree_promotions(starts_at);
CREATE INDEX idx_spree_properties_on_name ON spree_properties(name);
CREATE INDEX idx_spree_property_prototypes_on_property_id ON spree_property_prototypes(property_id);
CREATE INDEX idx_spree_property_prototypes_on_prototype_id ON spree_property_prototypes(prototype_id);
CREATE INDEX idx_spree_prototype_taxons_on_prototype_id ON spree_prototype_taxons(prototype_id);
CREATE INDEX idx_spree_prototype_taxons_on_prototype_id_and_taxon_id ON spree_prototype_taxons(prototype_id, taxon_id);
CREATE INDEX idx_spree_prototype_taxons_on_taxon_id ON spree_prototype_taxons(taxon_id);
CREATE UNIQUE INDEX idx_spree_refund_reasons_on_lower_name ON spree_refund_reasons(lower((name)::text));
CREATE INDEX idx_spree_refunds_on_payment_id ON spree_refunds(payment_id);
CREATE INDEX idx_spree_refunds_on_reimbursement_id ON spree_refunds(reimbursement_id);
CREATE INDEX idx_spree_reimbursement_credits_on_reimbursement_id ON spree_reimbursement_credits(reimbursement_id);
CREATE UNIQUE INDEX idx_spree_reimbursement_types_on_lower_name ON spree_reimbursement_types(lower((name)::text));
CREATE INDEX idx_spree_reimbursement_types_on_type ON spree_reimbursement_types(type);
CREATE INDEX idx_spree_reimbursements_on_customer_return_id ON spree_reimbursements(customer_return_id);
CREATE UNIQUE INDEX idx_spree_reimbursements_on_number ON spree_reimbursements(number);
CREATE INDEX idx_spree_reimbursements_on_order_id ON spree_reimbursements(order_id);
CREATE UNIQUE INDEX idx_spree_return_authorization_reasons_on_lower_name ON spree_return_authorization_reasons(lower((name)::text));
CREATE UNIQUE INDEX idx_spree_return_authorizations_on_number ON spree_return_authorizations(number);
CREATE INDEX idx_spree_return_authorizations_on_order_id ON spree_return_authorizations(order_id);
CREATE INDEX idx_spree_return_authorizations_on_stock_location_id ON spree_return_authorizations(stock_location_id);
CREATE INDEX idx_spree_return_items_on_exchange_variant_id ON spree_return_items(exchange_variant_id);
CREATE INDEX idx_spree_return_items_on_inventory_unit_id ON spree_return_items(inventory_unit_id);
CREATE INDEX idx_spree_return_items_on_override_reimbursement_type_id ON spree_return_items(override_reimbursement_type_id);
CREATE INDEX idx_spree_return_items_on_preferred_reimbursement_type_id ON spree_return_items(preferred_reimbursement_type_id);
CREATE INDEX idx_spree_return_items_on_reimbursement_id ON spree_return_items(reimbursement_id);
CREATE INDEX idx_spree_return_items_on_return_authorization_id ON spree_return_items(return_authorization_id);
CREATE INDEX idx_spree_role_users_on_role_id ON spree_role_users(role_id);
CREATE INDEX idx_spree_role_users_on_user_id ON spree_role_users(user_id);
CREATE UNIQUE INDEX idx_spree_role_users_pky ON spree_role_users(user_id, role_id);
CREATE UNIQUE INDEX idx_spree_roles_on_lower_name ON spree_roles(lower((name)::text));
CREATE INDEX idx_spree_shipments_on_address_id ON spree_shipments(address_id);
CREATE UNIQUE INDEX idx_spree_shipments_on_number ON spree_shipments(number);
CREATE INDEX idx_spree_shipments_on_order_id ON spree_shipments(order_id);
CREATE INDEX idx_spree_shipments_on_stock_location_id ON spree_shipments(stock_location_id);
CREATE INDEX idx_spree_shipping_categories_on_name ON spree_shipping_categories(name);
CREATE INDEX idx_spree_shipping_method_categories_on_shipping_category_id ON spree_shipping_method_categories(shipping_category_id);
CREATE INDEX idx_spree_shipping_method_categories_on_shipping_method_id ON spree_shipping_method_categories(shipping_method_id);
CREATE INDEX idx_spree_shipping_method_zones_on_shipping_method_id ON spree_shipping_method_zones(shipping_method_id);
CREATE INDEX idx_spree_shipping_method_zones_on_zone_id ON spree_shipping_method_zones(zone_id);
CREATE INDEX idx_spree_shipping_methods_on_deleted_at ON spree_shipping_methods(deleted_at);
CREATE INDEX idx_spree_shipping_methods_on_tax_category_id ON spree_shipping_methods(tax_category_id);
CREATE INDEX idx_spree_shipping_rates_on_selected ON spree_shipping_rates(selected);
CREATE INDEX idx_spree_shipping_rates_on_shipment_id ON spree_shipping_rates(shipment_id);
CREATE INDEX idx_spree_shipping_rates_on_shipping_method_id ON spree_shipping_rates(shipping_method_id);
CREATE INDEX idx_spree_shipping_rates_on_tax_rate_id ON spree_shipping_rates(tax_rate_id);
CREATE INDEX idx_spree_state_changes_on_stateful_id_and_stateful_type ON spree_state_changes(stateful_id, stateful_type);
CREATE INDEX idx_spree_states_on_country_id ON spree_states(country_id);
CREATE INDEX idx_spree_stock_items_on_backorderable ON spree_stock_items(backorderable);
CREATE INDEX idx_spree_stock_items_on_deleted_at ON spree_stock_items(deleted_at);
CREATE INDEX idx_spree_stock_items_on_stock_location_id ON spree_stock_items(stock_location_id);
CREATE INDEX idx_spree_stock_items_on_variant_id ON spree_stock_items(variant_id);
CREATE INDEX idx_spree_stock_locations_on_active ON spree_stock_locations(active);
CREATE INDEX idx_spree_stock_locations_on_backorderable_default ON spree_stock_locations(backorderable_default);
CREATE INDEX idx_spree_stock_locations_on_country_id ON spree_stock_locations(country_id);
CREATE INDEX idx_spree_stock_locations_on_propagate_all_variants ON spree_stock_locations(propagate_all_variants);
CREATE INDEX idx_spree_stock_locations_on_state_id ON spree_stock_locations(state_id);
CREATE INDEX idx_spree_stock_movements_on_stock_item_id ON spree_stock_movements(stock_item_id);
CREATE INDEX idx_spree_stock_transfers_on_destination_location_id ON spree_stock_transfers(destination_location_id);
CREATE UNIQUE INDEX idx_spree_stock_transfers_on_number ON spree_stock_transfers(number);
CREATE INDEX idx_spree_stock_transfers_on_source_location_id ON spree_stock_transfers(source_location_id);
CREATE INDEX idx_spree_store_credit_events_on_store_credit_id ON spree_store_credit_events(store_credit_id);
CREATE INDEX idx_spree_store_credit_types_on_priority ON spree_store_credit_types(priority);
CREATE INDEX idx_spree_store_credits_on_deleted_at ON spree_store_credits(deleted_at);
CREATE INDEX idx_spree_store_credits_on_type_id ON spree_store_credits(type_id);
CREATE INDEX idx_spree_store_credits_on_user_id ON spree_store_credits(user_id);
CREATE INDEX idx_spree_stores_on_default ON spree_stores("default");
CREATE UNIQUE INDEX idx_spree_stores_on_lower_code ON spree_stores(lower((code)::text));
CREATE INDEX idx_spree_stores_on_url ON spree_stores(url);
CREATE INDEX idx_spree_taggings_on_context ON spree_taggings(context);
CREATE INDEX idx_spree_taggings_on_tag_id ON spree_taggings(tag_id);
CREATE INDEX idx_spree_taggings_on_taggable_id ON spree_taggings(taggable_id);
CREATE INDEX idx_spree_taggings_on_taggable_type ON spree_taggings(taggable_type);
CREATE INDEX idx_spree_taggings_on_tagger_id ON spree_taggings(tagger_id);
CREATE INDEX idx_spree_taggings_on_tagger_id_and_tagger_type ON spree_taggings(tagger_id, tagger_type);
CREATE UNIQUE INDEX idx_spree_tags_on_name ON spree_tags(name);
CREATE INDEX idx_spree_tax_categories_on_deleted_at ON spree_tax_categories(deleted_at);
CREATE INDEX idx_spree_tax_categories_on_is_default ON spree_tax_categories(is_default);
CREATE INDEX idx_spree_tax_rates_on_deleted_at ON spree_tax_rates(deleted_at);
CREATE INDEX idx_spree_tax_rates_on_included_in_price ON spree_tax_rates(included_in_price);
CREATE INDEX idx_spree_tax_rates_on_show_rate_in_label ON spree_tax_rates(show_rate_in_label);
CREATE INDEX idx_spree_tax_rates_on_tax_category_id ON spree_tax_rates(tax_category_id);
CREATE INDEX idx_spree_tax_rates_on_zone_id ON spree_tax_rates(zone_id);
CREATE INDEX idx_spree_taxonomies_on_position ON spree_taxonomies("position");
CREATE INDEX idx_spree_taxons_on_lft ON spree_taxons(lft);
CREATE INDEX idx_spree_taxons_on_name ON spree_taxons(name);
CREATE INDEX idx_spree_taxons_on_position ON spree_taxons("position");
CREATE INDEX idx_spree_taxons_on_rgt ON spree_taxons(rgt);
CREATE INDEX idx_spree_trackers_on_active ON spree_trackers(active);
CREATE INDEX idx_spree_users_on_bill_address_id ON spree_users(bill_address_id);
CREATE INDEX idx_spree_users_on_deleted_at ON spree_users(deleted_at);
CREATE INDEX idx_spree_users_on_ship_address_id ON spree_users(ship_address_id);
CREATE INDEX idx_spree_users_on_spree_api_key ON spree_users(spree_api_key);
CREATE INDEX idx_spree_variants_on_deleted_at ON spree_variants(deleted_at);
CREATE INDEX idx_spree_variants_on_discontinue_on ON spree_variants(discontinue_on);
CREATE INDEX idx_spree_variants_on_is_master ON spree_variants(is_master);
CREATE INDEX idx_spree_variants_on_position ON spree_variants("position");
CREATE INDEX idx_spree_variants_on_product_id ON spree_variants(product_id);
CREATE INDEX idx_spree_variants_on_sku ON spree_variants(sku);
CREATE INDEX idx_spree_variants_on_tax_category_id ON spree_variants(tax_category_id);
CREATE INDEX idx_spree_variants_on_track_inventory ON spree_variants(track_inventory);
CREATE INDEX idx_spree_zone_members_on_zone_id ON spree_zone_members(zone_id);
CREATE INDEX idx_spree_zone_members_on_zoneable_id_and_zoneable_type ON spree_zone_members(zoneable_id, zoneable_type);
CREATE INDEX idx_spree_zones_on_default_tax ON spree_zones(default_tax);
CREATE INDEX idx_spree_zones_on_kind ON spree_zones(kind);
CREATE INDEX idx_spree_stock_movements_on_originator_id_and_originator_type ON spree_stock_movements(originator_id, originator_type);
CREATE INDEX idx_spree_taxons_on_parent_id ON spree_taxons(parent_id);
CREATE INDEX idx_spree_taxons_on_permalink ON spree_taxons(permalink);
CREATE INDEX idx_spree_taxons_on_taxonomy_id ON spree_taxons(taxonomy_id);
CREATE UNIQUE INDEX idx_spree_option_type_prototypes_prototype_id_option_type_id ON spree_option_type_prototypes(prototype_id, option_type_id);
CREATE UNIQUE INDEX idx_spree_shipping_rates_join_index ON spree_shipping_rates(shipment_id, shipping_method_id);
CREATE INDEX idx_spree_store_credit_events_originator ON spree_store_credit_events(originator_id, originator_type);
CREATE INDEX idx_spree_store_credits_originator ON spree_store_credits(originator_id, originator_type);
CREATE UNIQUE INDEX idx_spree_taggings_idx ON spree_taggings(tag_id, taggable_id, taggable_type, context, tagger_id, tagger_type);
CREATE INDEX idx_spree_taggings_idy ON spree_taggings(taggable_id, taggable_type, tagger_id, context);
CREATE INDEX idx_spree_stock_item_by_loc_and_var_id ON spree_stock_items(stock_location_id, variant_id);
CREATE UNIQUE INDEX idx_spree_shipping_method_and_categories ON spree_shipping_method_categories(shipping_category_id, shipping_method_id);
