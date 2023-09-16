-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "moderation_kicks" (
	"kick_id"	INTEGER,
	"target_id"	INTEGER NOT NULL,
	"moderator_id"	INTEGER NOT NULL,
	"kick_reason"	TEXT,
	FOREIGN KEY("target_id") REFERENCES "core_users"("user_id"),
	FOREIGN KEY("moderator_id") REFERENCES "core_users"("user_id"),
	PRIMARY KEY("kick_id")
);