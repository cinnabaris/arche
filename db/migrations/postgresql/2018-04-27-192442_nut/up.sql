CREATE TABLE votes (
  id BIGSERIAL PRIMARY KEY,
  resource_type VARCHAR(255) NOT NULL,
  resource_id BIGINT NOT NULL,
  point INT NOT NULL DEFAULT 0,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(), updated_at TIMESTAMP WITH TIME ZONE NOT NULL);

CREATE UNIQUE INDEX idx_votes_resources ON votes (resource_type, resource_id);

CREATE INDEX idx_votes_resource_type ON votes (resource_type);

CREATE TABLE attachments (
  id BIGSERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  url VARCHAR(255) NOT NULL,
  length INT NOT NULL,
  media_type VARCHAR(32) NOT NULL,
  resource_type VARCHAR(255) NOT NULL,
  resource_id BIGINT NOT NULL,
  user_id BIGINT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(), updated_at TIMESTAMP WITH TIME ZONE NOT NULL);

CREATE UNIQUE INDEX idx_attachments_url ON attachments (url);

CREATE INDEX idx_attachments_title ON attachments (title);

CREATE INDEX idx_attachments_resource_type ON attachments (resource_type);

CREATE INDEX idx_attachments_media_type ON attachments (media_type);

CREATE TABLE leave_words (
  id BIGSERIAL PRIMARY KEY,
  body TEXT NOT NULL,
  type VARCHAR(8) NOT NULL DEFAULT 'markdown',
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW());

CREATE TABLE links (
  id BIGSERIAL PRIMARY KEY,
  href VARCHAR(255) NOT NULL,
  label VARCHAR(255) NOT NULL,
  loc VARCHAR(16) NOT NULL,
  lang VARCHAR(8) NOT NULL,
  x INT NOT NULL DEFAULT 0,
  y INT NOT NULL DEFAULT 0,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(), updated_at TIMESTAMP WITH TIME ZONE NOT NULL);

CREATE INDEX idx_links_loc_lang ON links (loc, lang);

CREATE INDEX idx_links_lang ON links (lang);

CREATE TABLE cards (
  id BIGSERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  summary TEXT NOT NULL,
  type VARCHAR(8) NOT NULL DEFAULT 'html',
  action VARCHAR(32) NOT NULL,
  href VARCHAR(255) NOT NULL,
  logo VARCHAR(255) NOT NULL,
  loc VARCHAR(16) NOT NULL,
  lang VARCHAR(8) NOT NULL,
  sort INT NOT NULL DEFAULT 0,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(), updated_at TIMESTAMP WITH TIME ZONE NOT NULL);

CREATE INDEX idx_cards_loc_lang ON cards (loc, lang);

CREATE INDEX idx_cards_lang ON cards (lang);

CREATE TABLE friend_links (
  id BIGSERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  home VARCHAR(255) NOT NULL,
  logo VARCHAR(255) NOT NULL,
  sort INT NOT NULL DEFAULT 0,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(), updated_at TIMESTAMP WITH TIME ZONE NOT NULL);
