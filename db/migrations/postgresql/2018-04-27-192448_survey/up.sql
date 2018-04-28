CREATE TABLE survey_forms (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  body TEXT NOT NULL,
  type VARCHAR(8) NOT NULL DEFAULT 'markdown',
  uid VARCHAR(36) NOT NULL,
  mode VARCHAR(8) NOT NULL,
  user_id INT NOT NULL,
  nbf DATE NOT NULL DEFAULT current_date,
  exp DATE NOT NULL,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
);

CREATE INDEX idx_survey_forms_type ON survey_forms (type);

CREATE UNIQUE INDEX idx_survey_forms_uid ON survey_forms (uid);

CREATE TABLE survey_fields (
  id SERIAL PRIMARY KEY,
  label VARCHAR(255) NOT NULL,
  name VARCHAR(255) NOT NULL,
  value VARCHAR(255) NOT NULL,
  body TEXT NOT NULL,
  type VARCHAR(16) NOT NULL DEFAULT 'text',
  required BOOLEAN NOT NULL DEFAULT TRUE,
  form_id INT NOT NULL,
  sort_order INT NOT NULL DEFAULT 0,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
);

CREATE UNIQUE INDEX idx_survey_fields_name_form_id ON survey_fields (name, form_id);

CREATE TABLE survey_records (
  id SERIAL PRIMARY KEY,
  value TEXT NOT NULL,
  form_id INT NOT NULL,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
);
