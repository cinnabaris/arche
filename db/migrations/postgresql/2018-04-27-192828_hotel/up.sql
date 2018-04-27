CREATE TABLE hotel_rooms (
  id           BIGSERIAL PRIMARY KEY,
  uid          VARCHAR(8)                 NOT NULL,
  loc          VARCHAR(32)                NOT NULL,
  floor  VARCHAR(4)                NOT NULL,
  door    VARCHAR(8)                NOT NULL,
  bed    VARCHAR(4)                NOT NULL,
  status       VARCHAR(8) NOT NULL,
  description  TEXT,
  created_at   TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
  updated_at   TIMESTAMP WITHOUT TIME ZONE NOT NULL
);
CREATE UNIQUE INDEX idx_hotel_rooms_uid
  ON hotel_rooms (uid);
CREATE INDEX idx_hotel_rooms_loc
    ON hotel_rooms (loc);
CREATE INDEX idx_hotel_rooms_status
    ON hotel_rooms (status);

CREATE TABLE hotel_logs (
  id           BIGSERIAL PRIMARY KEY,
  member_id   BIGINT                NOT NULL,
  room_id BIGINT                NOT NULL,
  action  VARCHAR(8) NOT NULL,
  days SMALLINT,
  created_at   TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now()
);
