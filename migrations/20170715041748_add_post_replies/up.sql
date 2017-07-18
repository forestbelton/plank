ALTER TABLE posts ADD COLUMN reply_id INTEGER REFERENCES posts(id);

CREATE INDEX posts_reply_id_idx ON posts (reply_id);
