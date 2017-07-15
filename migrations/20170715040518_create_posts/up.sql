CREATE TABLE posts(
    id INTEGER PRIMARY KEY NOT NULL,
    uuid TEXT NOT NULL UNIQUE,
    author TEXT NOT NULL,
    create_date TIMESTAMP NOT NULL,
    body TEXT NOT NULL,
    attachment TEXT
);

CREATE INDEX posts_uuid_idx ON posts(uuid);
