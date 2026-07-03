CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    alias TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

ALTER TABLE posts ADD COLUMN user_id TEXT REFERENCES users(id);
ALTER TABLE mood_checkins ADD COLUMN user_id TEXT REFERENCES users(id);

CREATE INDEX IF NOT EXISTS idx_posts_user ON posts(user_id);
CREATE INDEX IF NOT EXISTS idx_mood_checkins_user ON mood_checkins(user_id);
