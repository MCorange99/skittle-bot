-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "moderation_message_references" (
	"message_reference"	BIGINT NOT NULL,
	"entry_id"			BIGINT NOT NULL,
	"message_id"		BIGINT NOT NULL,
	"note"				TEXT NOT NULL,
	FOREIGN KEY("message_id") REFERENCES "moderation_messages"("message_id"),
	PRIMARY KEY("message_reference","entry_id")
);