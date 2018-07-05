CREATE TABLE forum_categories (
    id SERIAL PRIMARY KEY,
    name varchar(64) NOT NULL,
    color char(6) DEFAULT 'AB9364' NOT NULL,
    description text,
    text_color char(6) DEFAULT 'FFFFFF' NOT NULL,
    "position" integer,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


CREATE TABLE forum_posts (
    id SERIAL PRIMARY KEY,
    user_id integer NOT NULL,
    topic_id integer NOT NULL,
    post_id integer,
    body text NOT NULL,
    type varchar(8) NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    deleted_at timestamp without time zone
);


CREATE TABLE forum_topics (
    id SERIAL PRIMARY KEY,
    title varchar NOT NULL,
    user_id integer NOT NULL,
    body text NOT NULL,
    type varchar(8) NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    deleted_at timestamp without time zone
);


CREATE TABLE forum_badges (
    id SERIAL PRIMARY KEY,
    name varchar(64) NOT NULL,
    description text NOT NULL,
    icon varchar(32) NOT NULL DEFAULT 'fa-certificate',
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


CREATE TABLE forum_categories_badges (
    id SERIAL PRIMARY KEY,
    category_id INTEGER NOT NULL,
    badge_id INTEGER NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE UNIQUE INDEX uk_forum_categories_badges_ids ON forum_categories_badges(category_id, badge_id);
CREATE INDEX idx_forum_categories_badges_category_id ON forum_categories_badges(category_id);
CREATE INDEX idx_forum_categories_badges_badge_id ON forum_categories_badges(badge_id);

CREATE TABLE forum_topics_badges (
    id SERIAL PRIMARY KEY,
    topic_id INTEGER NOT NULL,
    badge_id INTEGER NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);

CREATE UNIQUE INDEX uk_forum_topics_badges_ids ON forum_topics_badges(topic_id, badge_id);
CREATE INDEX idx_forum_topics_badges_category_id ON forum_topics_badges(topic_id);
CREATE INDEX idx_forum_topics_badges_badge_id ON forum_topics_badges(badge_id);
