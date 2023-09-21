-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "moderation_message_references" (
	"message_reference"	BIGINT,
	"entry_id"	BIGINT,
	"message_id"	BIGINT NOT NULL,
	"note"	TEXT,
	FOREIGN KEY("message_id") REFERENCES "moderation_messages"("message_id"),
	PRIMARY KEY("message_reference","entry_id")
);