CREATE TABLE library_books (
  id SERIAL PRIMARY KEY,
  uid VARCHAR(8) NOT NULL,
  title VARCHAR(255) NOT NULL,
  author VARCHAR(255) NOT NULL,
  publisher VARCHAR(255) NOT NULL,
  status SMALLINT NOT NULL,
  description TEXT,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
);

CREATE UNIQUE INDEX idx_library_books_uid ON library_books (uid);

CREATE INDEX idx_library_books_title ON library_books (title);

CREATE INDEX idx_library_books_author ON library_books (author);

CREATE TABLE library_logs (
  id SERIAL PRIMARY KEY,
  member_id INT NOT NULL,
  book_id INT NOT NULL,
  action VARCHAR(8) NOT NULL,
  days SMALLINT,
  description TEXT NOT NULL,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now()
);
