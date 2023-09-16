-- Your SQL goes here
CREATE TABLE moderation_kicks (
    kick_id INTEGER PRIMARY KEY NOT NULL,
    target_id INTEGER NOT NULL,
    moderator_id INTEGER NOT NULL,
    kick_reason TEXT NOT NULL
);