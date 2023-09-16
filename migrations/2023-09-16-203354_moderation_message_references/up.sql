-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "message_references" (
	"message_reference"	INTEGER,
	"entry_id"	INTEGER,
	"message_id"	INTEGER NOT NULL,
	"note"	TEXT,
	FOREIGN KEY("message_id") REFERENCES "moderation_messages"("message_id"),
	PRIMARY KEY("message_reference","entry_id")
);