CREATE TABLE cbeta_books (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  identifier VARCHAR(255) NOT NULL,
  language VARCHAR(32) NOT NULL,
  creator VARCHAR(255) NOT NULL,
  publisher VARCHAR(255),
  subject VARCHAR(255),
  description TEXT,
  published_at TIMESTAMP,
  version VARCHAR(5) NOT NULL,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
);

CREATE UNIQUE INDEX idx_cbeta_books_identifier ON cbeta_books (identifier);
CREATE INDEX idx_cbeta_books_creator ON cbeta_books (creator);
CREATE INDEX idx_cbeta_books_version ON cbeta_books (version);
CREATE INDEX idx_cbeta_books_language ON cbeta_books (language);

CREATE TABLE cbeta_pages (
  id SERIAL PRIMARY KEY,
  book_id INT NOT NULL,
  name VARCHAR(255) NOT NULL,
  href VARCHAR(255) NOT NULL,
  body BYTEA,
  media_type VARCHAR(255) NOT NULL,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
);

CREATE INDEX idx_cbeta_pages_media_type ON cbeta_pages (media_type);
CREATE INDEX idx_cbeta_pages_href_book ON cbeta_pages (href, book_id);
CREATE UNIQUE INDEX idx_cbeta_pages_name_book ON cbeta_pages (name, book_id);

CREATE TABLE cbeta_notes (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  book_id INT NOT NULL,
  body TEXT NOT NULL,
  media_type VARCHAR(8) NOT NULL DEFAULT 'markdown',
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
);

CREATE INDEX idx_cbeta_notes_media_type ON cbeta_notes (media_type);
