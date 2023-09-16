-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "moderation_messages" (
	"message_id"	INTEGER,
	"message_link"	TEXT NOT NULL,
	PRIMARY KEY("message_id")
);