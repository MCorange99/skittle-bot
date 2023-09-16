-- Your SQL goes here
CREATE TABLE moderation_reports (
    report_id INTEGER PRIMARY KEY NOT NULL,
    target_id INTEGER NOT NULL,
    moderator_id INTEGER NOT NULL,
    report_title TEXT NOT NULL,
    report_description TEXT NOT NULL,
    timestamp INTEGER NOT NULL -- changed from NUMERIC
);