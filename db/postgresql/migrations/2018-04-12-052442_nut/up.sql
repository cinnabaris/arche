CREATE TABLE votes (
  id SERIAL PRIMARY KEY,
  resource_type VARCHAR(255) NOT NULL,
  resource_id INT NOT NULL,
  "point" INT NOT NULL DEFAULT 0,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(), updated_at TIMESTAMP WITH TIME ZONE NOT NULL);

CREATE UNIQUE INDEX uk_votes_resources ON votes (resource_type, resource_id);

CREATE INDEX idx_votes_resource_type ON votes (resource_type);


CREATE TABLE leave_words (
  id SERIAL PRIMARY KEY,
  body TEXT NOT NULL,
  type VARCHAR(8) NOT NULL DEFAULT 'markdown',
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW());

CREATE TABLE links (
  id SERIAL PRIMARY KEY,
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
  id SERIAL PRIMARY KEY,
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
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  home VARCHAR(255) NOT NULL,
  logo VARCHAR(255) NOT NULL,
  sort INT NOT NULL DEFAULT 0,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(), updated_at TIMESTAMP WITH TIME ZONE NOT NULL);

  CREATE TABLE attachments (
      id SERIAL PRIMARY KEY,
      user_id INTEGER NOT NULL,
      name varchar(255) NOT NULL,
      size bigint NOT NULL,
      type varchar(64) NOT NULL,
      url varchar(255) NOT NULL,
      created_at timestamp without time zone NOT NULL,
      updated_at timestamp without time zone NOT NULL
  );

  CREATE UNIQUE INDEX uk_attachments_url ON attachments(url);
  CREATE INDEX idx_attachments_name ON attachments(name);
  CREATE INDEX idx_attachments_type ON attachments(type);
