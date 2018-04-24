CREATE TABLE currencies (
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

CREATE UNIQUE INDEX idx_currencies_key ON currencies(key);
CREATE INDEX idx_currencies_iso_code ON currencies(iso_code);
CREATE INDEX idx_currencies_name ON currencies(name);
