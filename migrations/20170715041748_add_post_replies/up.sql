ALTER TABLE posts ADD COLUMN reply_uuid TEXT REFERENCES posts(uuid);

CREATE INDEX posts_reply_uuid_idx ON posts (reply_uuid);
