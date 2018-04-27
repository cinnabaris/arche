CREATE TABLE donate_projects (
  id BIGSERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  body TEXT NOT NULL,
  type VARCHAR(8) NOT NULL,
  user_id BIGINT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);
CREATE INDEX idx_donate_projects_title ON donate_projects (title);
CREATE INDEX idx_donate_projects_type ON donate_projects (type);

CREATE TABLE donate_payments (
  id BIGSERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  summary VARCHAR(800) NOT NULL,
  type VARCHAR(16) NOT NULL,
  profile TEXT NOT NULL,
  project_id BIGINT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);
CREATE INDEX idx_donate_payments_title ON donate_payments (title);
CREATE INDEX idx_donate_payments_type ON donate_payments (type);
