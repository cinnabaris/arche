CREATE TABLE votes (
  id BIGSERIAL PRIMARY KEY,
  resource_type VARCHAR(32) NOT NULL,
  resource_id BIGINT NOT NULL,
  "point" INT NOT NULL DEFAULT 0,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
);

CREATE INDEX idx_votes_resource_type ON votes(resource_type);
CREATE UNIQUE INDEX idx_votes_resource_type_and_id ON votes(resource_type, resource_id);
