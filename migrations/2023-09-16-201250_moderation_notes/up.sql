-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "moderation_notes" (
	"note_id"	INTEGER,
	"target_id"	INTEGER NOT NULL,
	"moderator_id"	INTEGER NOT NULL,
	"note"	TEXT NOT NULL,
	"message_reference"	INTEGER,
	FOREIGN KEY("moderator_id") REFERENCES "core_users"("user_id"),
	FOREIGN KEY("target_id") REFERENCES "core_users"("user_id"),
	FOREIGN KEY("message_reference") REFERENCES "moderation_message_references"("message_reference"),
	PRIMARY KEY("note_id")
);