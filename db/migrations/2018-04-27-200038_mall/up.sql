
CREATE TABLE mall_addresses (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    first_name varchar(32) NOT NULL,
    last_name varchar(32) NOT NULL,
    content varchar(255) NOT NULL,
    zip_code varchar(16) NOT NULL,
    phone varchar(16) NOT NULL,
    company varchar,
    city varchar(32) NOT NULL,
    state_id INTEGER NOT NULL,
    country_id INTEGER NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE INDEX idx_mall_addresses_first_name ON mall_addresses(first_name);
CREATE INDEX idx_mall_addresses_last_name ON mall_addresses(last_name);
CREATE INDEX idx_mall_addresses_zip_code ON mall_addresses(zip_code);
CREATE INDEX idx_mall_addresses_phone ON mall_addresses(phone);
CREATE INDEX idx_mall_addresses_city ON mall_addresses(city);

CREATE TABLE mall_adjustment_reasons (
    id SERIAL PRIMARY KEY,
    name varchar(64) NOT NULL,
    code varchar(255) NOT NULL,
    active boolean DEFAULT true NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE UNIQUE INDEX uk_mall_adjustment_reasons_code on mall_adjustment_reasons(code);
CREATE INDEX idx_mall_adjustment_reasons_name on mall_adjustment_reasons(name);



CREATE TABLE mall_adjustments (
    id SERIAL PRIMARY KEY,
    resource_type varchar(255) NOT NULL,
    resource_id integer NOT NULL,
    reason TEXT NOT NULL,
    order_id INTEGER NOT NULL,
    amount numeric(10,2) NOT NULL,
    label varchar(255) NOT NULL,
    eligible boolean DEFAULT true NOT NULL,
    finalized boolean DEFAULT false NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE INDEX idx_mall_adjustments_label on mall_adjustments(label);
CREATE INDEX idx_mall_adjustments_resource_type on mall_adjustments(resource_type);

CREATE TABLE mall_assets (
    id SERIAL PRIMARY KEY,
    attachment_id INTEGER NOT NULL,
    "position" integer not null,
    alt VARCHAR(1024) NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);



CREATE TABLE mall_calculators (
    id SERIAL PRIMARY KEY,
    type varchar(16) NOT NULL,
    preferences text NOT NULL,
    resource_type varchar NOT NULL,
    resource_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE UNIQUE INDEX uk_mall_calculators_key_resource ON mall_calculators(type, resource_type, resource_id);
CREATE INDEX idx_mall_calculators_type ON mall_calculators(type);
CREATE INDEX idx_mall_calculators_resource_type ON mall_calculators(resource_type);


CREATE TABLE mall_countries (
    id SERIAL PRIMARY KEY,
    name varchar(255) NOT NULL,
    iso_name varchar(255) NOT NULL,
    iso2 varchar(2) NOT NULL,
    iso3 varchar(3) NOT NULL,
    code integer NOT NULL,
    states_required boolean DEFAULT false NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    created_at timestamp without time zone NOT NULL
);

CREATE UNIQUE INDEX uk_mall_countries_name ON mall_countries(lower(name));
CREATE UNIQUE INDEX uk_mall_countries_iso_name ON mall_countries(lower(iso_name));



CREATE TABLE mall_credit_cards (
    id SERIAL PRIMARY KEY NOT NULL,
    month varchar(2) NOT NULL,
    year varchar(4) NOT NULL,
    cc_type varchar(16) NOT NULL,
    last_digits varchar(8) NOT NULL,
    gateway_customer_profile_id varchar(1024) NOT NULL,
    gateway_payment_profile_id varchar(1024) NOT NULL,
    name varchar NOT NULL,
    user_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE UNIQUE INDEX uk_mall_credit_cards_name_user on mall_credit_cards(name, user_id);

CREATE TABLE mall_customer_returns (
    id SERIAL PRIMARY KEY,
    "number" varchar(255) NOT NULL,
    stock_location TEXT NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE UNIQUE INDEX uk_mall_customer_returns_number on mall_customer_returns("number");


CREATE TABLE mall_cartons (
    id SERIAL PRIMARY KEY,
    "number" varchar(255),
    stock_id integer NOT NULL,
    location VARCHAR(255) NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
CREATE UNIQUE INDEX idx_mall_cartons_stock_location on mall_cartons(stock, location);

CREATE TABLE mall_inventory_units (
    id SERIAL PRIMARY KEY,
    stock_id INTEGER NOT NULL,
    variant_id INTEGER NOT NULL,
    carton_id INTEGER NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);



CREATE TABLE mall_line_item_actions (
    id SERIAL PRIMARY KEY,
    line_item_id integer NOT NULL,
    action_id integer NOT NULL,
    quantity integer DEFAULT 0 NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


CREATE TABLE mall_line_items (
    id SERIAL PRIMARY KEY,
    variant_id integer NOT NULL,
    order_id integer NOT NULL,
    quantity integer NOT NULL,
    price numeric(10,2) NOT NULL,
    cost_price numeric(10,2) NOT NULL,
    tax_category_id integer NOT NULL,
    adjustment_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    additional_tax_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    promo_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    included_tax_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


CREATE TABLE mall_log_entries (
    id SERIAL PRIMARY KEY,
    resource_type varchar(255) NOT NULL,
    resource_id integer(255) NOT NULL,
    details text NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE INDEX idx_mall_log_entries_resource_type ON mall_log_entries(resource_type);
CREATE INDEX idx_mall_log_entries_resource_id ON mall_log_entries(resource_id);


CREATE TABLE mall_option_type_prototypes (
    id SERIAL PRIMARY KEY,
    prototype_id integer NOT NULL,
    option_type_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE UNIQUE INDEX uk_mall_option_type_prototypes on mall_option_type_prototypes(property_id, option_type_id);


CREATE TABLE mall_option_types (
    id SERIAL PRIMARY KEY,
    name varchar(64) NOT NULL,
    presentation varchar(1024) NOT NULL,
    "position" integer DEFAULT 0 NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE INDEX idx_mall_option_types_name ON mall_option_types(name);


CREATE TABLE mall_option_values (
    id SERIAL PRIMARY KEY,
    "position" integer NOT NULL,
    name varchar(64) NOT NULL,
    presentation varchar(1024) NOT NULL,
    option_type_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
CREATE INDEX idx_mall_option_values_name ON mall_option_values(name);


CREATE TABLE mall_option_values_variants (
    id SERIAL PRIMARY KEY,
    variant_id integer NOT NULL,
    option_value_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
CREATE UNIQUE INDEX uk_mall_option_values_variants ON mall_option_values_variants(variant_id, option_value_id);



CREATE TABLE mall_orders (
    id SERIAL PRIMARY KEY,
    "number" varchar(32) NOT NULL,
    item_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    total numeric(10,2) DEFAULT 0.0 NOT NULL,
    state varchar(32)  NOT NULL,
    adjustment_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    user_id integer NOT NULL,
    completed_at timestamp without time zone,
    bill_address varchar(255) NOT NULL,
    ship_address varchar(255) NOT NULL,
    payment_total numeric(10,2) DEFAULT 0.0,
    shipment_state varchar(8) NOT NULL,
    payment_state varchar(8) NOT NULL,
    email varchar(64) NOT NULL,
    special_instructions text,
    currency varchar(3) NOT NULL,
    shipment_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    additional_tax_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    promo_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    included_tax_total numeric(10,2) DEFAULT 0.0 NOT NULL,
    item_count integer DEFAULT 0 NOT NULL,
    approver_name varchar(255),
    approved_at timestamp without time zone,
    confirmation_delivered boolean DEFAULT false NOT NULL,
    guest_token varchar(255) NOT NULL,
    canceled_at timestamp without time zone,
    canceler_id integer,
    store_id integer NOT NULL,
    invoice_number varchar(255),
    invoiced_at timestamp without time zone,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


CREATE TABLE mall_orders_promotions (
    id SERIAL PRIMARY KEY,
    order_id integer NOT NULL,
    promotion_id integer NOT NULL,
    promotion_code_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE UNIQUE INDEX uk_mall_orders_promotions ON mall_orders_promotions(order_id, promotion_id, promotion_code_id);


CREATE TABLE mall_payment_capture_events (
    id SERIAL PRIMARY KEY,
    amount numeric(10,2) DEFAULT 0.0 NOT NULL,
    payment_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


CREATE TABLE mall_payment_methods (
    id SERIAL PRIMARY KEY,
    type varchar(8) NOT NULL,
    name varchar(32) NOT NULL,
    description text NOT NULL,
    active boolean DEFAULT true NOT NULL,
    auto_capture boolean NOT NULL,
    preferences text NOT NULL,
    "position" integer DEFAULT 0 NOT NULL,
    enable boolean DEFAULT true NOT NULL,
    deleted_at timestamp without time zone,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE UNIQUE INDEX uk_mall_payment_methods_name ON mall_payment_methods(name);
CREATE INDEX idx_mall_payment_methods_type ON mall_payment_methods(type);


CREATE TABLE mall_payments (
    id SERIAL PRIMARY KEY,
    amount numeric(10,2) DEFAULT 0.0 NOT NULL,
    order_id integer NOT NULL,
    payment_method_id INTEGER NOT NULL,
    state varchar(32) NOT NULL,
    response_code varchar(1024) NOT NULL,
    avs_response varchar(1024) NOT NULL,
    "number" varchar(255) NOT NULL,
    cvv_response_code varchar(1024),
    cvv_response_message varchar(1024),
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


CREATE TABLE mall_preferences (
    id SERIAL PRIMARY KEY,
    value text NOT NULL,
    key varchar(255) NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE UNIQUE INDEX uk_mall_preferences_key ON mall_preferences(key);


CREATE TABLE mall_prices (
    id SERIAL PRIMARY KEY,
    variant_id INTEGER NOT NULL,
    amount numeric(10,2) NOT NULL,
    currency varchar(3) NOT NULL,
    country_iso varchar(2) NOT NULL,
    deleted_at timestamp without time zone,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);



CREATE TABLE mall_product_option_types (
    id SERIAL PRIMARY KEY,
    "position" integer NOT NULL,
    product_id integer NOT NULL,
    option_type_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE UNIQUE INDEX uk_mall_product_option_types ON mall_product_option_types(product_id, option_type_id);


CREATE TABLE mall_product_promotion_rules (
    id SERIAL PRIMARY KEY,
    product_id integer,
    promotion_rule_id integer,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
CREATE UNIQUE INDEX uk_mall_product_promotion_rules ON mall_product_promotion_rules(product_id, promotion_rule_id);



CREATE TABLE mall_product_properties (
    id SERIAL PRIMARY KEY,
    value varchar(1024) NOT NULL,
    product_id integer NOT NULL,
    property_id integer NOT NULL,
    "position" integer DEFAULT 0,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
CREATE UNIQUE INDEX uk_mall_product_properties ON(product_id, property_id);


CREATE TABLE mall_products (
    id SERIAL PRIMARY KEY,
    name NOT NULL,
    description text NOT NULL,
    available_on timestamp without time zone NOT NULL,
    deleted_at timestamp without time zone,
    meta_description varchar(1024),
    meta_keywords varchar(255),
    meta_title varchar(255),
    tax_category_id integer NOT NULL,
    promotionable boolean DEFAULT true NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE INDEX idx_mall_products ON mall_products(name);




CREATE TABLE mall_products_taxons (
    id SERIAL PRIMARY KEY,
    product_id integer NOT NULL,
    taxon_id integer NOT NULL,
    "position" integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE UNIQUE INDEX uk_mall_products_taxons ON mall_products_taxons(product_id, taxon_id);


CREATE TABLE mall_promotion_action_line_items (
    id SERIAL PRIMARY KEY,
    promotion_action_id integer NOT NULL,
    variant_id integer NOT NULL,
    quantity integer DEFAULT 1 NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
CREATE UNIQUE INDEX uk_mall_promotion_action_line_items ON mall_promotion_action_line_items(promotion_action_id, variant_id);


CREATE TABLE mall_promotion_actions (
    id SERIAL PRIMARY KEY,
    promotion_id integer NOT NULL,
    "position" integer NOT NULL,
    type varchar NOT NULL,
    preferences text NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    deleted_at timestamp without time zone
);




CREATE TABLE mall_promotion_categories (
    id SERIAL PRIMARY KEY,
    name varchar(64) NOT NULL,
    code varchar(32) NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
CREATE UNIQUE INDEX uk_mall_promotion_categories_code ON mall_promotion_categories(code);
CREATE INDEX idx_mall_promotion_categories_namee ON mall_promotion_categories(name);



CREATE TABLE mall_promotion_code_batches (
    id SERIAL PRIMARY KEY,
    promotion_id integer not null,
    base_code varchar NOT NULL,
    number_of_codes integer NOT NULL,
    email varchar(255) ,
    error varchar(1024),
    state varchar(16) DEFAULT 'pending',
    join_characters char(1) DEFAULT '_' NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


CREATE TABLE mall_promotion_codes (
    id SERIAL PRIMARY KEY,
    promotion_id INTEGER NOT NULL,
    value varchar(255) NOT NULL,
    promotion_code_batch_id integer,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE UNIQUE INDEX idx_mall_promotion_codes ON mall_promotion_codes(product_id, value);



CREATE TABLE mall_promotion_rule_taxons (
    id SERIAL PRIMARY KEY,
    taxon_id integer,
    promotion_rule_id integer,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
CREATE UNIQUE INDEX uk_mall_promotion_rule_taxons ON mall_promotion_rule_taxons(taxon_id, promotion_rule_id);


CREATE TABLE mall_promotion_rules (
    id SERIAL PRIMARY KEY,
    promotion_id integer NOT NULL,
    product_group_id integer NOT NULL,
    type varchar(8) NOT NULL,
    code varchar(255) NOT NULL,
    preferences text NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE UNIQUE INDEX uk_mall_promotion_rules ON mall_promotion_rules(promotion_id, product_group_id);


CREATE TABLE mall_promotion_rules_stores (
    id SERIAL PRIMARY KEY,
    store_id integer NOT NULL,
    promotion_rule_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
CREATE UNIQUE INDEX uk_mall_promotion_rules_stores ON mall_promotion_rules_stores(store_id, promotion_rule_id);


CREATE TABLE mall_promotions (
    id SERIAL PRIMARY KEY,
    description varchar(1024) NOT NULL,
    expires_at timestamp without time zone,
    starts_at timestamp without time zone,
    name varchar(64) NOT NULL,
    type varchar(8) NOT NULL,
    usage_limit integer NOT NULL,
    match_policy varchar(16) DEFAULT 'all' NOT NULL,
    code varchar(255) NOT NULL,
    advertise boolean DEFAULT false NOT NULL,
    "path" varchar(32) NOT NULL,
    promotion_category_id integer NOT NULL,
    per_code_usage_limit integer NOT NULL,
    apply_automatically boolean DEFAULT false NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


CREATE TABLE mall_properties (
    id SERIAL PRIMARY KEY,
    name varchar(64) NOT NULL,
    presentation varchar(1024) NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);



CREATE TABLE mall_property_prototypes (
    id SERIAL PRIMARY KEY,
    prototype_id integer NOT NULL,
    property_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
CREATE UNIQUE INDEX uk_mall_property_prototypes on mall_property_prototypes(property_id, prototype_id);


CREATE TABLE mall_prototype_taxons (
    id SERIAL PRIMARY KEY,
    taxon_id integer NOT NULL,
    prototype_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
CREATE UNIQUE INDEX uk_mall_prototype_taxons ON mall_prototype_taxons(taxon_id, prototype_id);


CREATE TABLE mall_prototypes (
    id SERIAL PRIMARY KEY,
    name varchar(32) NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
CREATE UNIQUE INDEX uk_mall_prototypes ON mall_prototypes(name);



CREATE TABLE mall_refund_reasons (
    id SERIAL PRIMARY KEY,
    name varchar(255),
    active boolean DEFAULT true NOT NULL,
    mutable boolean DEFAULT true NOT NULL,
    code varchar(255) NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);



CREATE TABLE mall_refunds (
    id SERIAL PRIMARY KEY,
    payment_id integer NOT NULL,
    amount numeric(10,2) DEFAULT 0.0 NOT NULL,
    transaction_id varchar NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    refund_reason_id integer NOT NULL,
    reimbursement_id integer NOT NULL
);


CREATE TABLE mall_reimbursement_credits (
    id SERIAL PRIMARY KEY,
    amount numeric(10,2) DEFAULT 0.0 NOT NULL,
    reimbursement_id integer NOT NULL,
    creditable_id integer NOT NULL,
    creditable_type varchar NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
CREATE UNIQUE INDEX uk_mall_reimbursement_credits ON mall_reimbursement_credits(reimbursement_id, creditable_type, creditable_id);


CREATE TABLE mall_reimbursement_types (
    id SERIAL PRIMARY KEY,
    name varchar(32) NOT NULL,
    type varchar(8) NOT NULL,
    active boolean DEFAULT true NOT NULL,
    mutable boolean DEFAULT true NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);



CREATE TABLE mall_reimbursements (
    id SERIAL PRIMARY KEY,
    "number" varchar(255) NOT NULL,
    reimbursement_status varchar(16) NOT NULL,
    customer_return_id integer NOT NULL,
    order_id integer NOT NULL,
    total numeric(10,2) NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE UNIQUE INDEX uk_mall_reimbursements on mall_reimbursements("number");
CREATE UNIQUE INDEX uk_mall_reimbursements on mall_reimbursements(customer_return_id, order_id);

CREATE TABLE mall_relation_types (
    id SERIAL PRIMARY KEY,
    name varchar(32) NOT NULL,
    description text NOT NULL,
    applies_to varchar(255) NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


CREATE TABLE mall_relations (
    id SERIAL PRIMARY KEY,
    relation_type_id integer NOT NULL,
    relatable_type varchar NOT NULL,
    relatable_id integer NOT NULL,
    related_to_type varchar NOT NULL,
    related_to_id integer NOT NULL,
    "position" integer NOT NULL,
    discount_amount numeric(8,2) DEFAULT 0.0 NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);





CREATE TABLE mall_return_authorizations (
    id SERIAL PRIMARY KEY,
    "number" varchar(255) NOT NULL,
    state varchar(16) NOT NULL,
    order_id integer NOT NULL,
    memo text,
    stock_location_id integer NOT NULL,
    return_reason_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE UNIQUE INDEX uk_mall_return_authorizations_number ON mall_return_authorizations("number");



CREATE TABLE mall_return_items (
    id SERIAL PRIMARY KEY,
    return_authorization_id integer NOT NULL,
    inventory_unit_id integer NOT NULL,
    exchange_variant_id integer NOT NULL,
    amount numeric(12,4) DEFAULT 0.0 NOT NULL,
    included_tax_total numeric(12,4) DEFAULT 0.0 NOT NULL,
    additional_tax_total numeric(12,4) DEFAULT 0.0 NOT NULL,
    reception_status varchar(8) NOT NULL,
    acceptance_status varchar(8) NOT NULL,
    customer_return_id integer NOT NULL,
    reimbursement_id integer NOT NULL,
    exchange_inventory_unit_id integer NOT NULL,
    acceptance_status_errors text,
    preferred_reimbursement_type_id integer NOT NULL,
    override_reimbursement_type_id integer NOT NULL,
    resellable boolean DEFAULT true NOT NULL,
    return_reason_id integer,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


CREATE TABLE mall_return_reasons (
    id SERIAL PRIMARY KEY,
    name varchar(32) NOT NULL,
    active boolean DEFAULT true NOT NULL,
    mutable boolean DEFAULT true NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


CREATE TABLE mall_shipments (
    id SERIAL PRIMARY KEY,
    tracking varchar,
    number varchar,
    cost numeric(10,2) DEFAULT 0.0,
    shipped_at timestamp without time zone NOT NULL,
    order_id integer,
    deprecated_address_id integer,
    state varchar,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    stock_location_id integer,
    adjustment_total numeric(10,2) DEFAULT 0.0,
    additional_tax_total numeric(10,2) DEFAULT 0.0,
    promo_total numeric(10,2) DEFAULT 0.0,
    included_tax_total numeric(10,2) DEFAULT 0.0 NOT NULL
);


CREATE TABLE mall_shipping_method_stock_locations (
    id SERIAL PRIMARY KEY,
    shipping_method_id integer NOT NULL,
    stock_location_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
create unique index uk_mall_shipping_method_stock_locations on (shipping_method_id,stock_location_id);


CREATE TABLE mall_shipping_method_zones (
    id SERIAL PRIMARY KEY,
    shipping_method_id integer NOT NULL,
    zone_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

create unique index uk_mall_shipping_method_zones on mall_shipping_method_zones(shipping_method_id,zone_id);


CREATE TABLE mall_shipping_methods (
    id SERIAL PRIMARY KEY,
    name varchar(32) NOT NULL,
    tracking_url varchar(255),
    tax_category_id integer NOT NULL,
    code varchar(255) NOT NULL,
    available_to_all boolean DEFAULT true NOT NULL,
    carrier varchar NOT NULL,
    service_level varchar NOT NULL,
    available_to_users boolean DEFAULT true NOT NULL,
    deleted_at timestamp without time zone,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
CREATE unique index uk_mall_shipping_methods_code on mall_shipping_methods(code);


CREATE TABLE mall_shipping_rate_taxes (
    id SERIAL PRIMARY KEY,
    amount numeric(8,2) DEFAULT 0.0 NOT NULL,
    tax_rate_id integer NOT NULL,
    shipping_rate_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
create unique index uk_mall_shipping_rate_taxes on mall_shipping_rate_taxes(tax_rate_id,shipping_rate_id);


CREATE TABLE mall_shipping_rates (
    id SERIAL PRIMARY KEY,
    shipment_id integer NOT NULL,
    shipping_method_id integer NOT NULL,
    selected boolean DEFAULT false NOT NULL,
    cost numeric(8,2) DEFAULT 0.0 NOT NULL,
    tax_rate_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


CREATE TABLE mall_state_changes (
    id SERIAL PRIMARY KEY,
    name varchar(32) NOT NULL,
    previous_state varchar(8) NOT NULL,
    stateful_id integer NOT NULL,
    user_id integer NOT NULL,
    stateful_type varchar(255) NOT NULL,
    next_state varchar(8) NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);



CREATE TABLE mall_states (
    id SERIAL PRIMARY KEY,
    name varchar NOT NULL,
    abbr varchar NOT NULL,
    country_id integer NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    created_at timestamp without time zone NOT NULL
);





CREATE TABLE mall_stock_items (
    id SERIAL PRIMARY KEY,
    stock_location_id integer NOT NULL,
    variant_id integer NOT NULL,
    count_on_hand integer DEFAULT 0 NOT NULL,
    backorderable boolean DEFAULT false NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    deleted_at timestamp without time zone NOT NULL
);
create unique index uk_mall_stock_items(variant_id,stock_location_id);


CREATE TABLE mall_stock_locations (
    id SERIAL PRIMARY KEY,
    name varchar,
    "default" boolean DEFAULT false NOT NULL,
    address_id integer NOT NULL,
    active boolean DEFAULT true NOT NULL,
    backorderable_default boolean DEFAULT false NOT NULL,
    propagate_all_variants boolean DEFAULT true NOT NULL,
    "position" integer DEFAULT 0 NOT NULL,
    restock_inventory boolean DEFAULT true NOT NULL,
    fulfillable boolean DEFAULT true NOT NULL,
    code varchar(255) NOT NULL,
    check_stock_on_transfer boolean DEFAULT true NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
create unique index uk_mall_stock_locations_code on mall_stock_locations(code);


CREATE TABLE mall_stock_movements (
    id SERIAL PRIMARY KEY,
    stock_item_id integer NOT NULL,
    quantity integer DEFAULT 0 NOT NULL,
    action varchar(16) NOT NULL,
    originator_type varchar(255) NOT NULL,
    originator_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


CREATE TABLE mall_store_credit_events (
    id SERIAL PRIMARY KEY,
    store_credit_id INTEGER NOT NULL,
    action varchar NOT NULL,
    amount numeric(8,2) NOT NULL,
    user_total_amount numeric(8,2) DEFAULT 0.0 NOT NULL,
    authorization_code varchar NOT NULL,
    deleted_at timestamp without time zone NOT NULL,
    originator_type varchar(255) NOT NULL,
    originator_id integer(255) NOT NULL,
    update_reason_id integer NOT NULL,
    amount_remaining numeric(8,2) NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);



CREATE TABLE mall_store_credits (
    id SERIAL PRIMARY KEY,
    user_id integer NOT NULL,
    amount numeric(8,2) DEFAULT 0.0 NOT NULL,
    amount_used numeric(8,2) DEFAULT 0.0 NOT NULL,
    amount_authorized numeric(8,2) DEFAULT 0.0 NOT NULL,
    currency varchar(3) NOT NULL,
    memo text,
    type_id integer NOT NULL,
    deleted_at timestamp without time zone NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    invalidated_at date NOT NULL
);


CREATE TABLE mall_store_payment_methods (
    id SERIAL PRIMARY KEY,
    store_id integer not null,
    payment_method_id integer not null,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE UNIQUE index uk_mall_store_payment_methods on mall_store_payment_methods(store_id, payment_method_id);



CREATE TABLE mall_store_shipping_methods (
    id SERIAL PRIMARY KEY,
    store_id integer not null,
    shipping_method_id integer not null,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
CREATE UNIQUE index uk_mall_store_shipping_methods on mall_store_shipping_methods(store_id, shipping_method_id);


CREATE TABLE mall_stores (
    id SERIAL PRIMARY KEY,
    name varchar NOT NULL,
    url varchar NOT NULL,
    meta_description varchar(1024),
    meta_keywords varchar(255),
    meta_title varchar(255),
    mail_from_address_id integer NOT NULL,
    default_currency varchar(3) NOT NULL,
    code varchar(255) NOT NULL,
    "default" boolean DEFAULT false NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


CREATE TABLE mall_tax_rates (
    id SERIAL PRIMARY KEY,
    amount numeric(8,5) NOT NULL,
    zone_id integer NOT NULL,
    name varchar(255) NOT NULL,
    show_rate_in_label boolean DEFAULT true,
    deleted_at timestamp without time zone,
    starts_at timestamp without time zone NOT NULL,
    expires_at timestamp without time zone NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
);
CREATE INDEX idx_mall_tax_rates_name on mall_tax_rates(name);


CREATE TABLE mall_variants (
    id SERIAL PRIMARY KEY,
    sku varchar(255) NOT NULL,
    weight numeric(8,2) DEFAULT 0.0 NOT NULL,
    height numeric(8,2) NOT NULL,
    width numeric(8,2) NOT NULL,
    depth numeric(8,2) NOT NULL,
    is_master boolean DEFAULT false NOT NULL,
    product_id integer NOT NULL,
    cost_price numeric(10,2) NOT NULL,
    "position" integer NOT NULL,
    cost_currency varchar NOT NULL,
    track_inventory boolean DEFAULT true NOT NULL,
    tax_category_id integer NOT NULL,
    deleted_at timestamp without time zone,
    updated_at timestamp without time zone NOT NULL,
    created_at timestamp without time zone NOT NULL
);
create unique index uk_mall_variants_sku on mall_variants(sku);


CREATE TABLE mall_wishlists (
    id SERIAL PRIMARY KEY,
    variant_id integer not null,
    user_id integer not null,
    remark text,
    quantity integer DEFAULT 1 NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
create unique index uk_mall_wishlists on mall_wishlists(user_id, variant_id);


CREATE TABLE mall_zone_members (
    id SERIAL PRIMARY KEY,
    zoneable_type varchar(32) NOT NULL,
    zoneable_id integer NOT NULL,
    zone_id integer NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
create unique index uk_mall_zone_members on mall_zone_members(zoneable_id, zoneable_type, zone_id);

CREATE TABLE mall_zones (
    id SERIAL PRIMARY KEY,
    name varchar(255) NOT NULL,
    description text NOT NULL,
    zone_members_count integer DEFAULT 0 NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
create index idx_mall_zones_name on mall_zones(name);
