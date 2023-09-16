-- Your SQL goes here
CREATE TABLE moderation_mutes (
    mute_id INTEGER PRIMARY KEY NOT NULL,
    target_id INTEGER NOT NULL,
    moderator_id INTEGER NOT NULL,
    reason TEXT,
    timestamp NUMERIC NOT NULL,
    duration NUMERIC
);