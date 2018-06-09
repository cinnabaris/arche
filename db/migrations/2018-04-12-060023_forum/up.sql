CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name varchar(64) NOT NULL,
    color char(6) DEFAULT 'AB9364' NOT NULL,
    description text,
    text_color char(6) DEFAULT 'FFFFFF' NOT NULL,
    "position" integer,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    user_id integer NOT NULL,
    topic_id integer NOT NULL,
    post_id integer;
    body text NOT NULL,
    type varchar(8) NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    deleted_at timestamp without time zone
);


CREATE TABLE topics (
    id SERIAL PRIMARY KEY,
    title varchar NOT NULL,
    user_id integer NOT NULL,
    body text NOT NULL,
    type varchar(8) NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    deleted_at timestamp without time zone
);


CREATE TABLE badges (
    id SERIAL PRIMARY KEY,
    name varchar(64) NOT NULL,
    description text NOT NULL,
    icon varchar(32) NOT NULL DEFAULT 'fa-certificate',
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


CREATE TABLE categories_badges (
    id SERIAL PRIMARY KEY,
    category_id INTEGER NOT NULL,
    badge_id INTEGER NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE UNIQUE INDEX uk_categories_badges_ids ON categories_badges(category_id, badge_id);
CREATE INDEX idx_categories_badges_category_id ON categories_badges(category_id);
CREATE INDEX idx_categories_badges_badge_id ON categories_badges(badge_id);

CREATE TABLE topics_badges (
    id SERIAL PRIMARY KEY,
    topic_id INTEGER NOT NULL,
    badge_id INTEGER NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE UNIQUE INDEX uk_topics_badges_ids ON topics_badges(topic_id, badge_id);
CREATE INDEX idx_topics_badges_category_id ON topics_badges(topic_id);
CREATE INDEX idx_topics_badges_badge_id ON topics_badges(badge_id);



CREATE TABLE notifications (
    id SERIAL PRIMARY KEY,
    notification_type integer NOT NULL,
    user_id INTEGER NOT NULL,
    topic_id integer NOT NULL,
    body varchar(1024) NOT NULL,
    read boolean DEFAULT false NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


CREATE TABLE permalinks (
    id SERIAL PRIMARY KEY,
    url varchar(1000) NOT NULL,
    resource_id integer NOT NULL,
    resource_type varchar(255) NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE UNIQUE INDEX uk_permalinks_resource ON permalinks(resource_id, resource_type);
CREATE UNIQUE INDEX uk_permalinks_url ON permalinks(url);
CREATE UNIQUE INDEX uk_permalinks_resource_type ON permalinks(resource_type);


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
CREATE INDEX idx_attachments_name ON attachments(filename);
CREATE INDEX idx_attachments_mime ON attachments(mime);
