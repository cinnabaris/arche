CREATE TABLE members (
  id SERIAL PRIMARY KEY,
  email VARCHAR(36) NOT NULL,
  name VARCHAR(255) NOT NULL,
  address VARCHAR(255) NOT NULL,
  phone VARCHAR(255) NOT NULL,
  summary TEXT,
  deleted_at TIMESTAMP WITHOUT TIME ZONE,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
);

CREATE UNIQUE INDEX idx_members_email ON members (email);

CREATE INDEX idx_members_name ON members (name);

CREATE INDEX idx_members_phone ON members (phone);
