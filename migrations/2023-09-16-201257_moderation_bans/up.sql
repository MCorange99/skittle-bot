-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "moderation_bans" (
	"ban_id"	INTEGER,
	"target_id"	INTEGER NOT NULL,
	"moderator_id"	INTEGER NOT NULL,
	"ban_reason"	TEXT,
	"ban_duration"	NUMERIC,
	FOREIGN KEY("target_id") REFERENCES "core_users"("user_id"),
	FOREIGN KEY("moderator_id") REFERENCES "core_users"("user_id"),
	PRIMARY KEY("ban_id")
);