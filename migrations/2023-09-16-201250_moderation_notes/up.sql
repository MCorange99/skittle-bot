-- Your SQL goes here
CREATE TABLE moderation_notes (
      note_id INTEGER PRIMARY KEY NOT NULL,
      target_id INTEGER NOT NULL,
      moderator_id INTEGER NOT NULL,
      note_text TEXT NOT NULL
      -- removed since we note the user not a message
      -- message_reference INTEGER FOREIGN KEY
);