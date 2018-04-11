CREATE TABLE settings (
  id BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `key` VARCHAR(255) NOT NULL,
  value BLOB NOT NULL,
  salt BLOB,
  created_at DATETIME NOT NULL DEFAULT NOW(),
  updated_at DATETIME NOT NULL
);
CREATE UNIQUE INDEX idx_settings_key ON settings (`key`);
