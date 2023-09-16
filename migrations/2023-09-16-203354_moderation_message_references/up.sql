-- Your SQL goes here
CREATE TABLE moderation_message_references (
    entry_id INTEGER PRIMARY KEY NOT NULL,
    message_id INTEGER NOT NULL,
    note TEXT, -- wtf is this?

    FOREIGN KEY(message_id) REFERENCES moderation_messages(message_id)
);