CREATE TABLE posts(
    id BIGSERIAL PRIMARY KEY,
    uuid UUID NOT NULL UNIQUE,
    author TEXT NOT NULL,
    create_date TIMESTAMP WITH TIME ZONE NOT NULL,
    body TEXT NOT NULL,
    attachment TEXT
);

CREATE INDEX posts_uuid_idx ON posts(uuid);
