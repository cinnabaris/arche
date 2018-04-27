CREATE TABLE mall_currencies (
    id BIGSERIAL PRIMARY KEY,
    key VARCHAR(3) NOT NULL,
    iso_code varchar(3) NOT NULL,
    name varchar(255) NOT NULL,
    symbol varchar(8),
    alternate_symbols varchar(255) NOT NULL,
    subunit varchar(16),
    subunit_to_unit INT NOT NULL,
    symbol_first BOOLEAN NOT NULL DEFAULT FALSE,
    html_entity varchar(16),
    decimal_mark char(1) NOT NULL,
    thousands_separator char(1) NOT NULL,
    iso_numeric INT,
    smallest_denomination INT,
    updated_at timestamp without time zone NOT NULL
);

CREATE UNIQUE INDEX idx_mall_currencies_key ON mall_currencies(key);
CREATE INDEX idx_mall_currencies_iso_code ON mall_currencies(iso_code);
CREATE INDEX idx_mall_currencies_name ON mall_currencies(name);

CREATE TABLE mall_countries (
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
CREATE UNIQUE INDEX idx_mall_countries_on_lower_iso_name ON mall_countries(lower((iso_name)::text));
CREATE UNIQUE INDEX idx_mall_countries_on_lower_name ON mall_countries(lower((name)::text));


CREATE TABLE mall_states (
    id BIGSERIAL PRIMARY KEY,
    name varchar(255) NOT NULL,
    abbr varchar(32) NOT NULL,
    country_id BIGINT NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


CREATE TABLE mall_zones (
    id BIGSERIAL PRIMARY KEY,
    name varchar(255) NOT NULL,
    description TEXT NOT NULL,
    kind varchar(16) NOT NULL,
    default_tax boolean DEFAULT false NOT NULL,
    zone_members_count integer DEFAULT 0 NOT NULL,
    created_at timestamp without time zone DEFAULT NOW() NOT NULL,
    updated_at timestamp without time zone NOT NULL
);
