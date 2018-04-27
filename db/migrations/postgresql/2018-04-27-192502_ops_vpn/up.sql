CREATE TABLE vpn_users (
  id BIGSERIAL PRIMARY KEY,
  full_name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  summary TEXT,
  online BOOLEAN NOT NULL DEFAULT FALSE,
  enable BOOLEAN NOT NULL DEFAULT FALSE,
  nbf DATE NOT NULL DEFAULT current_date,
  exp DATE NOT NULL,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
);

CREATE UNIQUE INDEX idx_vpn_users_email ON vpn_users (email);

CREATE INDEX idx_vpn_users_full_name ON vpn_users (full_name);

CREATE TABLE vpn_logs (
  id BIGSERIAL PRIMARY KEY,
  user_id BIGINT NOT NULL,
  trusted_ip VARCHAR(39),
  trusted_port SMALLINT,
  remote_ip VARCHAR(39),
  remote_port SMALLINT,
  start_up TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  shut_down TIMESTAMP WITHOUT TIME ZONE,
  received FLOAT NOT NULL DEFAULT '0.0',
  send FLOAT NOT NULL DEFAULT '0.0'
);
