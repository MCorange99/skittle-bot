-- Your SQL goes here
CREATE TABLE moderation_bans (
    ban_id INTEGER PRIMARY KEY NOT NULL,
    target_id INTEGER NOT NULL,
    moderator_id INTEGER NOT NULL,
    ban_reason TEXT,
    ban_duration INTEGER
);