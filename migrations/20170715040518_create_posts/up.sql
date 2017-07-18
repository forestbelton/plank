CREATE TABLE posts(
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    author TEXT NOT NULL,
    create_date TIMESTAMP NOT NULL,
    body TEXT NOT NULL,
    attachment TEXT,
    reply_id INTEGER REFERENCES posts(id),
    source_addr TEXT NOT NULL
);

CREATE INDEX posts_uuid_idx ON posts(uuid);
CREATE INDEX posts_reply_id_idx ON posts (reply_id);