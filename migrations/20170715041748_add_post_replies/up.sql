ALTER TABLE posts ADD COLUMN reply_uuid UUID;

CREATE INDEX posts_reply_uuid_idx ON posts (reply_uuid);
