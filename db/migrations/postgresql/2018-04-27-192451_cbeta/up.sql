CREATE TABLE cbeta_books (
  id SERIAL PRIMARY KEY,
  uid VARCHAR(64) NOT NULL,
  author VARCHAR(255) NOT NULL,
  publisher VARCHAR(255) NOT NULL,
  title VARCHAR(255) NOT NULL,
  type VARCHAR(36) NOT NULL,
  lang VARCHAR(32) NOT NULL,
  subject VARCHAR(255),
  description TEXT,
  published_at DATE NOT NULL DEFAULT current_date,
  cover VARCHAR(255),
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
);

CREATE UNIQUE INDEX idx_cbeta_books_uid ON cbeta_books (uid);
CREATE INDEX idx_cbeta_books_author ON cbeta_books (author);
CREATE INDEX idx_cbeta_books_publisher ON cbeta_books (publisher);
CREATE INDEX idx_cbeta_books_type ON cbeta_books (type);
CREATE INDEX idx_cbeta_books_lang ON cbeta_books (lang);


CREATE TABLE cbeta_pages (
  id SERIAL PRIMARY KEY,
  name INT NOT NULL,
  book_id INT NOT NULL,
  media_type VARCHAR(255) NOT NULL,
  body BYTEA NOT NULL,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
);

CREATE INDEX idx_cbeta_pages_media_type ON cbeta_pages (media_type);
CREATE UNIQUE INDEX idx_cbeta_pages_name_book ON cbeta_pages (name, book_id);

CREATE TABLE cbeta_notes (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  page_id INT NOT NULL,
  body TEXT NOT NULL,
  type VARCHAR(8) NOT NULL DEFAULT 'markdown',
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
);

CREATE INDEX idx_cbeta_notes_type ON cbeta_notes (type);
