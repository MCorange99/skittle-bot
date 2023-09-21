-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "moderation_notes" (
	"note_id"	BIGINT,
	"target_id"	BIGINT NOT NULL,
	"moderator_id"	BIGINT NOT NULL,
	"note"	TEXT NOT NULL,
	"message_reference"	BIGINT,
	FOREIGN KEY("moderator_id") REFERENCES "core_users"("user_id"),
	FOREIGN KEY("target_id") REFERENCES "core_users"("user_id"),
	FOREIGN KEY("message_reference") REFERENCES "moderation_message_references"("message_reference"),
	PRIMARY KEY("note_id")
);